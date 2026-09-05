//! Demand-driven traversal of an I3S scene layer via the [`kiba::Loader`]
//! protocol.
//!
//! # What varies between scene layers
//!
//! "An I3S scene layer" is several quite different index structures
//! wearing one name, and a loader that handles only one of them handles
//! almost no real data:
//!
//! * **Node-paged** (1.7 and later, `3DObject` / `IntegratedMesh`). The
//!   tree lives in `nodepages/{n}`, fixed-size pages of
//!   [`Node`](crate::cmn::Node) records addressed by a global node index.
//!   Children are node indices, so expanding a node may require a page
//!   that has not been fetched.
//! * **Point** (`psl`, 1.7 and later). Node-paged in exactly the same way
//!   — its pages hold `cmn::Node` — but the page index is spelled
//!   `pointNodePages` and the layer declares a single, always-Draco
//!   `geometryDefinition` rather than an array.
//! * **Point cloud** (`pcsl`). Also node-paged, but the page documents
//!   hold [`NodePcsl`](crate::pcsl::NodePcsl) records whose children are a
//!   `firstChild`/`childCount` *run* rather than an explicit list, and
//!   whose payload is addressed by `resourceId`.
//! * **Building** (`bld`). Has no nodes of its own; it is a tree of
//!   sublayers, each geometry-bearing one being a complete scene layer
//!   under `/sublayers/{id}`. Expanding the root descends into them, so
//!   every structure above composes beneath a building layer.
//! * **Legacy** (1.6, and early `3DObject`). No node pages at all:
//!   each node is its own `3dNodeIndexDocument` and children are named by
//!   relative `href`.
//!
//! [`SceneLayerLoader`] composes all of these behind one type. Which one
//! is in play is decided once, when the layer document is fetched, from
//! the presence of `nodePages` and the layer's `layerType` — never guessed
//! per node.
//!
//! # What varies between sources
//!
//! Orthogonally to the index structure, the *same* layer may be served
//! over REST, read out of a `.slpk` archive, or read from an exploded
//! package directory. Those differ in more than a prefix — resources gain
//! `.json`/`.bin` extensions and possibly `.gz` inside a package — so all
//! naming goes through [`Source`], and this module never formats a URL
//! itself.
//!
//! # Laziness
//!
//! [`SceneLayerLoader::open`] fetches nothing. The layer document is
//! fetched on the first call to [`Loader::root`] and retained; node pages
//! are fetched on demand and retained. A [`NodeRef`] is an `Arc` clone
//! plus a small index, so handing them around costs nothing.

use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, Mutex},
};

use ::kiba::{
    ExpandFuture, Expansion, Fetch, FetchRequest, LoadFuture, LoadOutcome, Loader, RootFuture,
};

use crate::{
    Error, Source, bld,
    cmn::{self, SceneLayerInfo},
    pcsl, psl,
    select::{GeometryEncodingPreference, select_buffer_in_definition},
    urls::{page_id_of, page_offset_of},
};

// ---------------------------------------------------------------------
// Layer state
// ---------------------------------------------------------------------

/// The parsed layer document, in whichever profile it was written.
///
/// The profiles are not subsets of one another — each has its own store, node
/// record and attribute layout — so each is kept as a distinct variant rather
/// than coerced into [`SceneLayerInfo`] with empty fields.
#[derive(Debug)]
enum LayerDocument {
    /// A `3DObject` or `IntegratedMesh` layer.
    Mesh(Box<SceneLayerInfo>),
    /// A `Point` (`psl`) layer.
    ///
    /// Point layers are paged like mesh layers and share `cmn::NodePage`,
    /// but declare a single `geometryDefinition` and name their page index
    /// `pointNodePages`.
    Point(Box<psl::SceneLayerInfoPsl>),
    /// A point-cloud (`pcsl`) layer.
    PointCloud(Box<pcsl::LayerPcsl>),
    /// A `Building` (`bld`) layer, which owns no nodes of its own.
    ///
    /// A building layer is a tree of [`bld::SublayerBld`]s; the geometry
    /// lives in the non-`group` sublayers, each of which is a complete scene
    /// layer addressed below `/sublayers/{id}`.
    Building(Box<bld::LayerBld>),
}

/// How this layer's node tree is indexed.
#[derive(Debug, Clone, PartialEq)]
enum IndexKind {
    /// 1.7+ node pages of [`cmn::Node`], used by mesh and point layers.
    NodePaged { nodes_per_page: u64, root: u64 },
    /// `pcsl` node pages of [`pcsl::NodePcsl`].
    PointCloudPaged { nodes_per_page: u64 },
    /// 1.6 per-node `3dNodeIndexDocument`s.
    Legacy,
    /// A building layer, whose "children" are its sublayers rather than
    /// nodes. Each entry is a sublayer id to descend into.
    Sublayers { ids: Vec<u32> },
}

/// A fetched node page, in whichever profile the layer uses.
#[derive(Debug)]
enum NodePage {
    Mesh(cmn::NodePage),
    PointCloud(pcsl::NodePageDefinitionPcsl),
}

/// Everything retained for the lifetime of a loader: the layer document,
/// the resolved index shape, and the pages fetched so far.
struct LayerState {
    source: Source,
    document: LayerDocument,
    index: IndexKind,
    preference: GeometryEncodingPreference,
    pages: Mutex<HashMap<u64, Arc<NodePage>>>,
    /// Legacy `3dNodeIndexDocument`s, keyed by resolved URI.
    legacy_nodes: Mutex<HashMap<String, Arc<cmn::NodeIndexDocument>>>,
}

impl std::fmt::Debug for LayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayerState")
            .field("source", &self.source)
            .field("index", &self.index)
            .finish_non_exhaustive()
    }
}

impl LayerState {
    /// Assembles a layer with empty caches.
    fn new(
        source: Source,
        document: LayerDocument,
        index: IndexKind,
        preference: GeometryEncodingPreference,
    ) -> Self {
        Self {
            source,
            document,
            index,
            preference,
            pages: Mutex::new(HashMap::new()),
            legacy_nodes: Mutex::new(HashMap::new()),
        }
    }

    /// The address of this layer's root node.
    ///
    /// For a legacy layer the root is named by `store.rootNode` — a relative
    /// URL such as `./nodes/root` — and is emphatically *not* node 0. Real
    /// 1.6 services publish the root under the literal name `root`, so
    /// assuming an index here fetches a URL that does not exist.
    fn root_address(&self) -> NodeAddress {
        match &self.index {
            IndexKind::NodePaged { root, .. } => NodeAddress::Paged { index: *root },
            IndexKind::PointCloudPaged { .. } => NodeAddress::Paged { index: 0 },
            IndexKind::Sublayers { .. } => NodeAddress::Sublayers,
            IndexKind::Legacy => NodeAddress::Legacy {
                uri: match self.root_node_href() {
                    Some(href) => resolve_relative(&self.source.layer(), href),
                    // Without `store.rootNode` there is nothing to resolve,
                    // so fall back to the conventional node 0.
                    None => self.source.node(0),
                },
            },
        }
    }

    /// The layer's `store.rootNode`, in whichever profile declares one.
    fn root_node_href(&self) -> Option<&str> {
        match &self.document {
            LayerDocument::Mesh(layer) => layer.store.root_node.as_deref(),
            LayerDocument::Point(layer) => layer.store.root_node.as_deref(),
            // `pcsl` is always node-paged and `bld` has no nodes at all.
            LayerDocument::PointCloud(_) | LayerDocument::Building(_) => None,
        }
    }

    /// The mesh-profile layer document, if this is a mesh layer.
    fn mesh(&self) -> Option<&SceneLayerInfo> {
        match &self.document {
            LayerDocument::Mesh(layer) => Some(layer),
            _ => None,
        }
    }

    /// The layer's attribute declarations, whichever profile it uses.
    ///
    /// All four profiles spell this the same way and mean the same thing, so
    /// the callers that build attribute URLs need not know the profile.
    fn attribute_storage_info(&self) -> &[cmn::AttributeStorageInfo] {
        match &self.document {
            LayerDocument::Mesh(layer) => &layer.attribute_storage_info,
            LayerDocument::Point(layer) => &layer.attribute_storage_info,
            // Point clouds declare `AttributeInfoPcsl`, a different type, so
            // they are handled by `point_cloud_contents` instead.
            LayerDocument::PointCloud(_) | LayerDocument::Building(_) => &[],
        }
    }
}

// ---------------------------------------------------------------------
// Fetching and parsing
// ---------------------------------------------------------------------

/// Fetches raw resources and caches parsed documents.
struct Fetcher {
    fetch: Fetch,
}

impl Fetcher {
    fn new(fetch: Fetch) -> Self {
        Self { fetch }
    }

    /// Fetches the bytes at `uri`, performing no caching or parsing.
    async fn bytes(&self, uri: impl Into<String>) -> Result<Vec<u8>, Error> {
        let uri = uri.into();
        let response = (self.fetch)(FetchRequest {
            uri: uri.clone(),
            range: None,
        })
        .await
        .map_err(|source| Error::fetch(uri, source))?;
        Ok(response.bytes)
    }

    /// Fetches and deserializes a JSON document.
    async fn json<T: serde::de::DeserializeOwned>(&self, uri: String) -> Result<T, Error> {
        let bytes = self.bytes(uri.clone()).await?;
        serde_json::from_slice(&bytes).map_err(|source| Error::parse(uri, source.to_string()))
    }
}

// ---------------------------------------------------------------------
// Item, content reference and content
// ---------------------------------------------------------------------

/// Which retained state a [`NodeRef`] addresses, and how.
#[derive(Clone)]
enum NodeAddress {
    /// A node in a mesh or point-cloud node page, by global node index.
    Paged { index: u64 },
    /// A legacy `3dNodeIndexDocument`, by resolved URI.
    Legacy { uri: String },
    /// The root of a building layer, which stands in for the layer itself.
    ///
    /// A building layer has no nodes; expanding this address descends into
    /// each geometry-bearing sublayer and yields *that* sublayer's root.
    Sublayers,
}

/// A lightweight, stable handle to one node of a scene layer.
///
/// Cloning a `NodeRef` clones an `Arc` and a small address; it never
/// copies node data, and never holds a borrow of a page.
#[derive(Clone)]
pub struct NodeRef {
    layer: Arc<LayerState>,
    address: NodeAddress,
    depth: usize,
}

impl NodeRef {
    /// The node's depth below the layer root, which is depth 0.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// The node's global index, for node-paged layers.
    pub fn index(&self) -> Option<u64> {
        match self.address {
            NodeAddress::Paged { index } => Some(index),
            _ => None,
        }
    }

    /// The node document's URI, for legacy layers.
    pub fn uri(&self) -> Option<&str> {
        match &self.address {
            NodeAddress::Legacy { uri } => Some(uri),
            _ => None,
        }
    }

    /// The building layer this node stands for, when it is a building root.
    pub fn building(&self) -> Option<&bld::LayerBld> {
        match &self.layer.document {
            LayerDocument::Building(layer) => Some(layer),
            _ => None,
        }
    }

    /// The point-profile layer document, when this is a point layer.
    pub fn point_layer(&self) -> Option<&psl::SceneLayerInfoPsl> {
        match &self.layer.document {
            LayerDocument::Point(layer) => Some(layer),
            _ => None,
        }
    }

    /// The layer document this node belongs to, when it is a mesh layer.
    pub fn layer(&self) -> Option<&SceneLayerInfo> {
        self.layer.mesh()
    }
}

impl std::fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("NodeRef");
        match &self.address {
            NodeAddress::Paged { index } => s.field("index", index),
            NodeAddress::Legacy { uri } => s.field("uri", uri),
            NodeAddress::Sublayers => s.field("sublayers", &true),
        };
        s.field("depth", &self.depth).finish()
    }
}

/// What a [`ContentRef`] points at.
///
/// A node's payload is not one blob. Geometry, per-feature attributes and
/// textures are separately addressed resources with different decoders, and
/// a caller usually wants to fetch some but not all of them — so the kind
/// travels with the reference rather than being inferred from the URI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentKind {
    /// A geometry buffer, decodable with [`crate::decode_geometry`] unless
    /// `compressed` is set, in which case it is Draco.
    Geometry {
        /// Index into `SceneLayerInfo::geometry_definitions`.
        definition: usize,
        /// Index of the buffer within that definition.
        buffer: usize,
        /// Whether the payload is Draco-compressed.
        compressed: bool,
        /// Declared vertex count, when the node page states one.
        vertex_count: Option<u32>,
        /// Declared feature count, when the node page states one.
        feature_count: Option<u32>,
    },
    /// A binary attribute buffer, decodable with
    /// [`crate::decode_attribute`].
    Attribute {
        /// The `f_N` key this buffer holds.
        key: String,
        /// The attribute's user-visible name.
        name: String,
    },
    /// An encoded texture image. The bytes are passed through untouched;
    /// this crate does not decode image formats.
    Texture {
        /// The format name from the texture set definition.
        name: String,
        /// The declared image format.
        format: cmn::Format,
    },
    /// A legacy 1.6 resource named only by `href`, whose role is given by
    /// the array it appeared in.
    Legacy {
        /// `"geometry"`, `"attribute"`, `"feature"` or `"shared"`.
        role: &'static str,
    },
}

/// A resolved reference to one content resource of a node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentRef {
    /// The resolved resource location.
    pub uri: String,
    /// What the resource is.
    pub kind: ContentKind,
}

/// Raw bytes loaded for a [`ContentRef`].
///
/// Decoding is left to the caller so that a client which only needs, say,
/// geometry does not pay to decode attributes, and so that a client with
/// its own Draco decoder can handle compressed buffers.
#[derive(Debug, Clone)]
pub struct LoadedContent {
    /// The resolved resource location.
    pub uri: String,
    /// The resource bytes.
    pub bytes: Vec<u8>,
    /// What the resource is.
    pub kind: ContentKind,
}

// ---------------------------------------------------------------------
// The loader
// ---------------------------------------------------------------------

/// A [`kiba::Loader`] over an I3S scene layer.
///
/// ```no_run
/// use i3s::{SceneLayerLoader, Source};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let loader = SceneLayerLoader::open(
///     Source::rest("https://example.com/SceneServer", 0),
///     |_request| async { unimplemented!("caller-provided transport") },
/// );
/// # let _ = loader;
/// # Ok(())
/// # }
/// ```
pub struct SceneLayerLoader {
    source: Source,
    preference: GeometryEncodingPreference,
    fetcher: Arc<Fetcher>,
    /// The layer document, fetched at most once on the first `root` call.
    layer: Arc<Mutex<Option<Arc<LayerState>>>>,
}

impl SceneLayerLoader {
    /// Creates a loader without fetching anything.
    pub fn open<F, Fut>(source: Source, fetch: F) -> Self
    where
        F: Fn(FetchRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<::kiba::FetchResponse, ::kiba::FetchError>> + Send + 'static,
    {
        let fetch: Fetch = Arc::new(move |request| Box::pin(fetch(request)));
        Self {
            source,
            preference: GeometryEncodingPreference::default(),
            fetcher: Arc::new(Fetcher::new(fetch)),
            layer: Arc::new(Mutex::new(None)),
        }
    }

    /// Selects the compressed geometry buffer where one exists.
    ///
    /// Only useful to a caller with its own Draco decoder;
    /// [`crate::decode_geometry`] rejects compressed buffers.
    pub fn with_geometry_preference(mut self, preference: GeometryEncodingPreference) -> Self {
        self.preference = preference;
        self
    }
}

/// The resolved layer state, fetching and parsing the layer document only
/// on the first call.
///
/// This is a free function rather than a method so that `Loader::root`'s
/// future can own everything it needs instead of borrowing the loader.
async fn resolve_layer(
    fetcher: &Fetcher,
    source: &Source,
    preference: GeometryEncodingPreference,
    slot: &Mutex<Option<Arc<LayerState>>>,
) -> Result<Arc<LayerState>, Error> {
    if let Some(state) = slot.lock().unwrap().clone() {
        return Ok(state);
    }
    let state = Arc::new(fetch_layer(fetcher, source, preference).await?);
    // A concurrent caller may have won the race; prefer whichever state is
    // already published so every `NodeRef` shares one page cache.
    let mut slot = slot.lock().unwrap();
    Ok(slot.get_or_insert(state).clone())
}

/// Fetches the layer document and resolves its index structure.
async fn fetch_layer(
    fetcher: &Fetcher,
    source: &Source,
    preference: GeometryEncodingPreference,
) -> Result<LayerState, Error> {
    let uri = source.layer();
    let bytes = fetcher.bytes(uri.clone()).await?;

    // `layerType` decides the profile. Every profile declares it, and the
    // allowed values are disjoint across profiles, so one probe suffices.
    let probe: LayerTypeProbe = serde_json::from_slice(&bytes)
        .map_err(|error| Error::parse(uri.clone(), error.to_string()))?;
    let parse = |error: serde_json::Error| Error::parse(uri.clone(), error.to_string());

    match probe.layer_type.as_deref() {
        Some("PointCloud") => {
            let layer: pcsl::LayerPcsl = serde_json::from_slice(&bytes).map_err(parse)?;
            let nodes_per_page = u64::from(layer.store.index.nodes_per_page);
            if nodes_per_page == 0 {
                return Err(Error::InvalidNodesPerPage {
                    layer_id: layer.id,
                    nodes_per_page: 0,
                });
            }
            Ok(LayerState::new(
                source.clone().with_layer_id(layer.id),
                LayerDocument::PointCloud(Box::new(layer)),
                IndexKind::PointCloudPaged { nodes_per_page },
                preference,
            ))
        }

        Some("Point") => {
            let layer: psl::SceneLayerInfoPsl = serde_json::from_slice(&bytes).map_err(parse)?;
            let layer_id = layer.id;
            // "For legacy purposes, this property is called pointNodePages"
            // -- 3DSceneLayer.psl.md. It is an ordinary nodePageDefinition.
            let index = paged_index(layer.point_node_pages.as_ref(), layer_id)?;
            Ok(LayerState::new(
                source.clone().with_layer_id(layer_id),
                LayerDocument::Point(Box::new(layer)),
                index,
                preference,
            ))
        }

        Some("Building") => {
            let layer: bld::LayerBld = serde_json::from_slice(&bytes).map_err(parse)?;
            let layer_id = layer.id;
            // A building layer owns no nodes; its content is reached by
            // descending into each non-`group` sublayer.
            let mut ids = Vec::new();
            collect_geometry_sublayers(&layer.sublayers, &mut ids);
            Ok(LayerState::new(
                source.clone().with_layer_id(layer_id),
                LayerDocument::Building(Box::new(layer)),
                IndexKind::Sublayers { ids },
                preference,
            ))
        }

        _ => {
            let layer: SceneLayerInfo = serde_json::from_slice(&bytes).map_err(parse)?;
            let layer_id = layer.id;
            let index = match layer.node_pages.as_ref() {
                Some(pages) => paged_index(Some(pages), layer_id)?,
                // Absent `nodePages` is not an error: it is how a 1.6 layer
                // says its tree is a graph of `3dNodeIndexDocument`s.
                None => IndexKind::Legacy,
            };
            Ok(LayerState::new(
                source.clone().with_layer_id(layer_id),
                LayerDocument::Mesh(Box::new(layer)),
                index,
                preference,
            ))
        }
    }
}

/// Builds a [`IndexKind::NodePaged`] from a `nodePageDefinition`.
fn paged_index(pages: Option<&cmn::NodePageDefinition>, layer_id: u32) -> Result<IndexKind, Error> {
    let Some(pages) = pages else {
        return Ok(IndexKind::Legacy);
    };
    let nodes_per_page = u64::from(pages.nodes_per_page);
    if nodes_per_page == 0 {
        return Err(Error::InvalidNodesPerPage {
            layer_id,
            nodes_per_page: 0,
        });
    }
    Ok(IndexKind::NodePaged {
        nodes_per_page,
        root: u64::from(pages.root_index.unwrap_or(0)),
    })
}

/// Collects the ids of every sublayer that holds geometry.
///
/// `group` sublayers are organisational only — they carry no resources of
/// their own — but they nest, so the tree is walked rather than scanned:
///
/// > Sublayers contained in this layer.
/// >
/// > -- `sublayer.bld.md`
fn collect_geometry_sublayers(sublayers: &[bld::SublayerBld], ids: &mut Vec<u32>) {
    for sublayer in sublayers {
        if sublayer.layer_type == bld::SublayerBldLayerType::Group {
            collect_geometry_sublayers(&sublayer.sublayers, ids);
        } else {
            ids.push(sublayer.id);
        }
    }
}

/// Just enough of a layer document to choose a profile before committing
/// to a full deserialization.
#[derive(serde::Deserialize)]
struct LayerTypeProbe {
    #[serde(rename = "layerType")]
    layer_type: Option<String>,
}

impl Loader for SceneLayerLoader {
    type Item = NodeRef;
    type ContentRef = ContentRef;
    type Content = LoadedContent;
    type Error = Error;

    fn root(&self) -> RootFuture<Self::Item, Self::Error> {
        let source = self.source.clone();
        let preference = self.preference;
        let fetcher = self.fetcher.clone();
        let slot = self.layer.clone();
        Box::pin(async move {
            let layer = resolve_layer(&fetcher, &source, preference, &slot).await?;
            let address = layer.root_address();
            Ok(NodeRef {
                layer,
                address,
                depth: 0,
            })
        })
    }

    fn expand(&self, item: Self::Item) -> ExpandFuture<Self::Item, Self::ContentRef, Self::Error> {
        let fetcher = self.fetcher.clone();
        let preference = self.preference;
        Box::pin(async move {
            match item.address.clone() {
                NodeAddress::Paged { index } => {
                    expand_paged(&fetcher, item.layer, index, item.depth).await
                }
                NodeAddress::Legacy { uri } => {
                    expand_legacy(&fetcher, item.layer, uri, item.depth).await
                }
                NodeAddress::Sublayers => {
                    expand_sublayers(&fetcher, item.layer, item.depth, preference).await
                }
            }
        })
    }

    fn load(&self, content: Self::ContentRef) -> LoadFuture<Self::Content, Self::Error> {
        let fetcher = self.fetcher.clone();
        Box::pin(async move {
            let bytes = fetcher.bytes(content.uri.clone()).await?;
            if bytes.is_empty() {
                return Ok(LoadOutcome::Empty);
            }
            Ok(LoadOutcome::Ready(LoadedContent {
                uri: content.uri,
                bytes,
                kind: content.kind,
            }))
        })
    }
}

// ---------------------------------------------------------------------
// Expansion
// ---------------------------------------------------------------------

/// Returns the node page containing `index`, fetching it once.
async fn page_for(
    fetcher: &Fetcher,
    layer: &Arc<LayerState>,
    index: u64,
) -> Result<(Arc<NodePage>, u64, usize), Error> {
    let nodes_per_page = match &layer.index {
        IndexKind::NodePaged { nodes_per_page, .. }
        | IndexKind::PointCloudPaged { nodes_per_page } => *nodes_per_page,
        IndexKind::Legacy | IndexKind::Sublayers { .. } => {
            return Err(Error::NoNodePages {
                layer_id: layer.source.layer_id(),
            });
        }
    };
    let page_id = page_id_of(index, nodes_per_page);
    let offset = page_offset_of(index, nodes_per_page);

    if let Some(page) = layer.pages.lock().unwrap().get(&page_id).cloned() {
        return Ok((page, page_id, offset));
    }

    let uri = layer.source.node_page(page_id);
    let page = match layer.index {
        IndexKind::PointCloudPaged { .. } => {
            NodePage::PointCloud(fetcher.json::<pcsl::NodePageDefinitionPcsl>(uri).await?)
        }
        _ => NodePage::Mesh(fetcher.json::<cmn::NodePage>(uri).await?),
    };
    let page = Arc::new(page);
    layer
        .pages
        .lock()
        .unwrap()
        .insert(page_id, Arc::clone(&page));
    Ok((page, page_id, offset))
}

/// Expands a node addressed by global index, in either paged profile.
async fn expand_paged(
    fetcher: &Fetcher,
    layer: Arc<LayerState>,
    index: u64,
    depth: usize,
) -> Result<Expansion<NodeRef, ContentRef>, Error> {
    let (page, page_id, offset) = page_for(fetcher, &layer, index).await?;
    match page.as_ref() {
        NodePage::Mesh(page) => {
            let node = page.nodes.get(offset).ok_or(Error::MissingNode {
                node_index: index,
                page_id,
            })?;
            let children = node
                .children
                .iter()
                .map(|&child| NodeRef {
                    layer: Arc::clone(&layer),
                    address: NodeAddress::Paged {
                        index: u64::from(child),
                    },
                    depth: depth + 1,
                })
                .collect();
            Ok(Expansion {
                children,
                contents: mesh_contents(&layer, node),
            })
        }
        NodePage::PointCloud(page) => {
            let node = page.nodes.get(offset).ok_or(Error::MissingNode {
                node_index: index,
                page_id,
            })?;
            // Point-cloud children are a contiguous run, not a list.
            let first = u64::from(node.first_child);
            let children = (0..u64::from(node.child_count))
                .map(|n| NodeRef {
                    layer: Arc::clone(&layer),
                    address: NodeAddress::Paged { index: first + n },
                    depth: depth + 1,
                })
                .collect();
            Ok(Expansion {
                children,
                contents: point_cloud_contents(&layer, node),
            })
        }
    }
}

/// The content references of one 1.7+ node, in the mesh or point profile.
///
/// Both profiles use `cmn::Node` and `cmn::AttributeStorageInfo`; they
/// differ only in how a geometry buffer is chosen, which
/// [`geometry_content`] resolves.
fn mesh_contents(layer: &LayerState, node: &cmn::Node) -> Vec<ContentRef> {
    let Some(mesh) = node.mesh.as_ref() else {
        return Vec::new();
    };
    let mut contents = Vec::new();

    if let Some(geometry) = mesh.geometry.as_ref()
        && let Some(content) = geometry_content(layer, geometry)
    {
        contents.push(content);
    }

    if let Some(attribute) = mesh.attribute.as_ref() {
        let resource = attribute.resource;
        for storage in layer.attribute_storage_info() {
            // `key` is `f_N`; the URL wants the bare `N`, and a key that
            // is not of that shape is not addressable, so it is skipped
            // rather than producing a URI that cannot resolve.
            let Some(key) = storage.key.strip_prefix("f_") else {
                continue;
            };
            let Ok(key_index) = key.parse::<usize>() else {
                continue;
            };
            contents.push(ContentRef {
                uri: layer.source.attribute(resource, key_index),
                kind: ContentKind::Attribute {
                    key: storage.key.to_string(),
                    name: storage.name.to_string(),
                },
            });
        }
    }

    // Only the mesh profile declares texture sets; point layers symbolize
    // through `resources/`, which is not node-scoped.
    if let Some(info) = layer.mesh()
        && let Some(material) = mesh.material.as_ref()
        && let Some(resource) = material.resource
    {
        for set in &info.texture_set_definitions {
            for format in &set.formats {
                contents.push(ContentRef {
                    uri: layer.source.texture(
                        resource,
                        &format.name,
                        texture_extension(format.format),
                    ),
                    kind: ContentKind::Texture {
                        name: format.name.to_string(),
                        format: format.format,
                    },
                });
            }
        }
    }

    contents
}

/// Resolves a node's geometry reference against its layer's definitions.
///
/// The mesh profile indexes an array of definitions and may offer both an
/// uncompressed and a Draco buffer; the point profile declares exactly one
/// definition whose buffers, per `geometryDefinition.psl.md`, "must be
/// compressed", so there is nothing to select between.
fn geometry_content(layer: &LayerState, geometry: &cmn::MeshGeometry) -> Option<ContentRef> {
    let definition = geometry.definition as usize;
    let (buffer, compressed) = match &layer.document {
        LayerDocument::Mesh(info) => {
            let def = info.geometry_definitions.get(definition)?;
            let buffer = select_buffer_in_definition(def, layer.preference)?;
            (
                buffer,
                def.geometry_buffers[buffer].compressed_attributes.is_some(),
            )
        }
        LayerDocument::Point(info) => {
            // Point geometry buffers are always Draco -- the profile's
            // `geometryBuffer.psl` declares no uncompressed form -- so
            // there is nothing to select between, only to look up.
            let def = info.geometry_definitions.get(definition)?;
            if def.geometry_buffers.is_empty() {
                return None;
            }
            (0, true)
        }
        LayerDocument::PointCloud(_) | LayerDocument::Building(_) => return None,
    };
    Some(ContentRef {
        uri: layer.source.geometry(geometry.resource, buffer),
        kind: ContentKind::Geometry {
            definition,
            buffer,
            compressed,
            vertex_count: geometry.vertex_count,
            feature_count: geometry.feature_count,
        },
    })
}

/// Expands a building layer into the roots of its geometry sublayers.
///
/// A building scene layer stores no nodes itself. Each non-`group`
/// sublayer is a complete scene layer rooted at
/// `/layers/{bim_layer_id}/sublayers/{id}`, so this fetches each one's own
/// layer document and yields its root — after which every existing
/// traversal path applies unchanged.
async fn expand_sublayers(
    fetcher: &Fetcher,
    layer: Arc<LayerState>,
    depth: usize,
    preference: GeometryEncodingPreference,
) -> Result<Expansion<NodeRef, ContentRef>, Error> {
    let IndexKind::Sublayers { ids } = &layer.index else {
        return Ok(Expansion {
            children: Vec::new(),
            contents: Vec::new(),
        });
    };

    let mut children = Vec::with_capacity(ids.len());
    for &id in ids {
        let source = layer.source.sublayer(id);
        let sublayer = Arc::new(fetch_layer(fetcher, &source, preference).await?);
        let address = sublayer.root_address();
        children.push(NodeRef {
            layer: sublayer,
            address,
            depth: depth + 1,
        });
    }

    Ok(Expansion {
        children,
        contents: Vec::new(),
    })
}

/// The content references of one point-cloud node.
fn point_cloud_contents(layer: &LayerState, node: &pcsl::NodePcsl) -> Vec<ContentRef> {
    let LayerDocument::PointCloud(info) = &layer.document else {
        return Vec::new();
    };
    let resource = node.resource_id;
    let mut contents = vec![ContentRef {
        // A point-cloud node's positions live in geometry buffer 0; the
        // profile defines no alternative encodings.
        uri: layer.source.geometry(resource, 0),
        kind: ContentKind::Geometry {
            definition: 0,
            buffer: 0,
            compressed: false,
            vertex_count: node.vertex_count,
            feature_count: None,
        },
    }];

    for storage in &info.attribute_storage_info {
        let Some(key) = storage.key.strip_prefix("f_") else {
            continue;
        };
        let Ok(key_index) = key.parse::<usize>() else {
            continue;
        };
        contents.push(ContentRef {
            uri: layer.source.attribute(resource, key_index),
            kind: ContentKind::Attribute {
                key: storage.key.to_string(),
                name: storage.name.to_string(),
            },
        });
    }
    contents
}

/// Expands a legacy 1.6 `3dNodeIndexDocument`.
async fn expand_legacy(
    fetcher: &Fetcher,
    layer: Arc<LayerState>,
    uri: String,
    depth: usize,
) -> Result<Expansion<NodeRef, ContentRef>, Error> {
    let cached = layer.legacy_nodes.lock().unwrap().get(&uri).cloned();
    let document = match cached {
        Some(document) => document,
        None => {
            let document = Arc::new(fetcher.json::<cmn::NodeIndexDocument>(uri.clone()).await?);
            layer
                .legacy_nodes
                .lock()
                .unwrap()
                .insert(uri.clone(), Arc::clone(&document));
            document
        }
    };

    let children = document
        .children
        .iter()
        .filter_map(|child| child.href.as_deref())
        .map(|href| NodeRef {
            layer: Arc::clone(&layer),
            address: NodeAddress::Legacy {
                uri: resolve_relative(&uri, href),
            },
            depth: depth + 1,
        })
        .collect();

    let mut contents = Vec::new();
    for (resources, role) in [
        (&document.geometry_data, "geometry"),
        (&document.attribute_data, "attribute"),
        (&document.feature_data, "feature"),
    ] {
        for resource in resources {
            contents.push(ContentRef {
                uri: resolve_relative(&uri, &resource.href),
                kind: ContentKind::Legacy { role },
            });
        }
    }
    if let Some(shared) = document.shared_resource.as_ref() {
        contents.push(ContentRef {
            uri: resolve_relative(&uri, &shared.href),
            kind: ContentKind::Legacy { role: "shared" },
        });
    }

    Ok(Expansion { children, contents })
}

/// Resolves a legacy `href` against the URI of the document that held it.
///
/// I3S 1.6 hrefs are relative paths like `./0/1` or `../nodes/2`, and they
/// chain: a node reached by `./1` resolves its own children relative to
/// itself. The base is therefore treated as a directory unless its last
/// segment names a file, which is how the same rule covers both a REST
/// node URL (`.../nodes/root`) and a packaged document
/// (`nodes/root/3dNodeIndexDocument.json`).
///
/// This is deliberately not a general URI resolver: the forms the
/// specification's examples use are the forms handled, and anything
/// absolute is passed through unchanged.
fn resolve_relative(base: &str, href: &str) -> String {
    if href.contains("://") || href.starts_with('/') {
        return href.to_string();
    }

    // Keep the scheme and authority out of the segment arithmetic so that
    // a `..` can never climb past the host.
    let (prefix, path) = match base.split_once("://") {
        Some((scheme, rest)) => match rest.split_once('/') {
            Some((authority, path)) => (format!("{scheme}://{authority}"), path),
            None => (format!("{scheme}://{rest}"), ""),
        },
        None => (String::new(), base),
    };

    let mut segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    // A trailing segment with an extension is a file name, not a
    // directory, so it is dropped before appending the relative path.
    if segments.last().is_some_and(|last| last.contains('.')) {
        segments.pop();
    }

    for segment in href.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            other => segments.push(other),
        }
    }

    let joined = segments.join("/");
    if prefix.is_empty() {
        joined
    } else {
        format!("{prefix}/{joined}")
    }
}

/// The file extension a texture format uses inside a package.
fn texture_extension(format: cmn::Format) -> &'static str {
    match format {
        cmn::Format::Jpg => "jpg",
        // PNG textures are stored with a plain `.bin` extension.
        cmn::Format::Png => "bin",
        cmn::Format::Dds => "dds",
        cmn::Format::KtxEtc2 => "ktx",
        cmn::Format::Ktx2 => "ktx2",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_dot_relative_hrefs() {
        assert_eq!(
            resolve_relative("https://example.com/layers/0/nodes/root", "./1"),
            "https://example.com/layers/0/nodes/root/1"
        );
    }

    #[test]
    fn resolves_parent_relative_hrefs() {
        assert_eq!(
            resolve_relative("https://example.com/layers/0/nodes/root", "../2"),
            "https://example.com/layers/0/nodes/2"
        );
    }

    #[test]
    fn passes_absolute_hrefs_through() {
        assert_eq!(
            resolve_relative("https://example.com/a", "https://other/b"),
            "https://other/b"
        );
    }

    #[test]
    fn resolves_package_relative_hrefs() {
        // Inside a package the base has no scheme, so the result must
        // stay a bare archive-relative path.
        assert_eq!(
            resolve_relative("nodes/root/3dNodeIndexDocument.json", "./1"),
            "nodes/root/1"
        );
    }

    #[test]
    fn texture_extensions_follow_the_spec_table() {
        assert_eq!(texture_extension(cmn::Format::Jpg), "jpg");
        assert_eq!(texture_extension(cmn::Format::Png), "bin");
        assert_eq!(texture_extension(cmn::Format::Dds), "dds");
        assert_eq!(texture_extension(cmn::Format::KtxEtc2), "ktx");
        assert_eq!(texture_extension(cmn::Format::Ktx2), "ktx2");
    }
}

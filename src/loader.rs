//! Demand-driven traversal of an I3S scene layer via the [`hiera::Loader`]
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
//!   - its pages hold `cmn::Node` - but the page index is spelled
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
//! the presence of `nodePages` and the layer's `layerType` - never guessed
//! per node.
//!
//! # What varies between sources
//!
//! Orthogonally to the index structure, the *same* layer may be served
//! over REST, read out of a `.slpk` archive, or read from an exploded
//! package directory. Those differ in more than a prefix - resources gain
//! `.json`/`.bin` extensions and possibly `.gz` inside a package - so all
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

use ::hiera::{
    DescribeFuture, ExpandFuture, Expansion, FetchRequest, LoadFuture, LoadOutcome, Loader,
    RootFuture, Spawner,
};

use crate::{
    Error, FetchStore, SceneLayerStore, Source, bld,
    cmn::{self, SceneLayerInfo},
    pcsl, psl,
    select::{GeometryEncodingPreference, select_buffer_in_definition},
    urls::{page_id_of, page_offset_of},
};

/// The parsed layer document, in whichever profile it was written.
///
/// The profiles are not subsets of one another - each has its own store, node
/// record and attribute layout - so each is kept as a distinct variant rather
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

/// Profile-specific traversal behavior composed by [`SceneLayerLoader`].
///
/// The outer loader owns source transport and parsed-page caches. This
/// delegate owns only the profile's hierarchy interpretation, so adding a
/// profile cannot turn the shared transport facade into another format model.
#[derive(Debug, Clone, PartialEq)]
enum ProfileLoader {
    /// 1.7+ MeshPyramids pages of [`cmn::Node`].
    MeshPyramids { nodes_per_page: u64, root: u64 },
    /// Point pages of [`cmn::Node`].
    Point { nodes_per_page: u64, root: u64 },
    /// `pcsl` node pages of [`pcsl::NodePcsl`].
    PointCloudPaged { nodes_per_page: u64 },
    /// 1.6 per-node `3dNodeIndexDocument`s.
    Legacy,
    /// A building layer, whose "children" are its sublayers rather than
    /// nodes. Each entry is a sublayer id to descend into.
    Sublayers { ids: Vec<u32> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageFormat {
    Common,
    PointCloud,
}

impl ProfileLoader {
    fn root_address(&self, source: &Source, root_node_href: Option<&str>) -> NodeAddress {
        match self {
            Self::MeshPyramids { root, .. } | Self::Point { root, .. } => {
                NodeAddress::Paged { index: *root }
            }
            Self::PointCloudPaged { .. } => NodeAddress::Paged { index: 0 },
            Self::Sublayers { .. } => NodeAddress::Sublayers,
            Self::Legacy => NodeAddress::Legacy {
                uri: match root_node_href {
                    Some(href) => resolve_relative(&source.layer(), href),
                    None => source.node(0),
                },
            },
        }
    }

    fn page_format(&self) -> Option<(u64, PageFormat)> {
        match self {
            Self::MeshPyramids { nodes_per_page, .. } | Self::Point { nodes_per_page, .. } => {
                Some((*nodes_per_page, PageFormat::Common))
            }
            Self::PointCloudPaged { nodes_per_page } => {
                Some((*nodes_per_page, PageFormat::PointCloud))
            }
            Self::Legacy | Self::Sublayers { .. } => None,
        }
    }

    fn sublayer_ids(&self) -> Option<&[u32]> {
        match self {
            Self::Sublayers { ids } => Some(ids),
            _ => None,
        }
    }
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
    profile: ProfileLoader,
    preference: GeometryEncodingPreference,
    pages: Mutex<HashMap<u64, Arc<NodePage>>>,
    /// Legacy `3dNodeIndexDocument`s, keyed by resolved URI.
    legacy_nodes: Mutex<HashMap<String, Arc<cmn::NodeIndexDocument>>>,
}

impl std::fmt::Debug for LayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayerState")
            .field("source", &self.source)
            .field("profile", &self.profile)
            .finish_non_exhaustive()
    }
}

impl LayerState {
    /// Assembles a layer with empty caches.
    fn new(
        source: Source,
        document: LayerDocument,
        profile: ProfileLoader,
        preference: GeometryEncodingPreference,
    ) -> Self {
        Self {
            source,
            document,
            profile,
            preference,
            pages: Mutex::new(HashMap::new()),
            legacy_nodes: Mutex::new(HashMap::new()),
        }
    }

    /// The address of this layer's root node.
    ///
    /// For a legacy layer the root is named by `store.rootNode` - a relative
    /// URL such as `./nodes/root` - and is emphatically *not* node 0. Real
    /// 1.6 services publish the root under the literal name `root`, so
    /// assuming an index here fetches a URL that does not exist.
    fn root_address(&self) -> NodeAddress {
        self.profile
            .root_address(&self.source, self.root_node_href())
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

/// Fetches raw resources and caches parsed documents.
struct Fetcher {
    store: Arc<dyn SceneLayerStore>,
    spawner: Arc<dyn Spawner>,
}

impl Fetcher {
    fn new(store: Arc<dyn SceneLayerStore>, spawner: Arc<dyn Spawner>) -> Self {
        Self { store, spawner }
    }

    /// Fetches the bytes at `uri`, performing no caching or parsing.
    async fn bytes(&self, uri: impl Into<String>) -> Result<::hiera::Bytes, Error> {
        let uri = uri.into();
        self.store.read(uri.into()).await
    }

    async fn entries(&self) -> Result<Option<Vec<crate::ResourcePath>>, Error> {
        self.store.entries().await
    }

    /// Fetches and deserializes a JSON document.
    ///
    /// Deserialization runs through this fetcher's [`Spawner`] (an
    /// [`::hiera::InlineSpawner`] by default), so applications may opt into
    /// running this CPU work off whatever thread polls the loader's future.
    async fn json<T>(&self, uri: String) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        let bytes = self.bytes(uri.clone()).await?;
        let parsed = ::hiera::spawn(&*self.spawner, move || {
            serde_json::from_slice(&bytes).map_err(|source| Error::parse(uri, source.to_string()))
        })
        .await
        .map_err(Error::spawn)?;
        parsed
    }
}

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

/// Source-native bounds reported for one [`NodeRef`].
#[derive(Debug, Clone, PartialEq)]
pub enum NodeBounds {
    /// An oriented box in the layer's declared spatial reference.
    OrientedBox(cmn::Obb),
    /// A minimum bounding sphere in the layer's declared spatial reference.
    Sphere([f64; 4]),
    /// An organizational node with no spatial extent of its own.
    Unbounded,
}

/// Immutable native facts for a [`NodeRef`].
///
/// This intentionally carries only selection-relevant facts, rather than
/// cloning the layer or a full retained node document.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeInfo {
    /// The node's source-native spatial bound.
    pub bounds: NodeBounds,
    /// The source-provided level-of-detail metric, when present.
    pub lod_threshold: Option<f64>,
    /// Depth below the scene-layer root.
    pub depth: usize,
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
/// a caller usually wants to fetch some but not all of them - so the kind
/// travels with the reference rather than being inferred from the URI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentKind {
    /// A geometry buffer, decodable with [`crate::UncompressedGeometryCodec`] unless
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
    /// [`crate::StandardAttributeCodec`].
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
    pub bytes: ::hiera::Bytes,
    /// What the resource is.
    pub kind: ContentKind,
}

/// A [`hiera::Loader`] over an I3S scene layer.
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
    ///
    /// CPU parsing (layer/node-page/legacy-node JSON) runs inline on
    /// whatever thread polls this loader's futures, identical to before
    /// [`Spawner`] existed. Use [`Self::open_with_spawner`] to opt into
    /// routing that work elsewhere.
    pub fn open<F, Fut>(source: Source, fetch: F) -> Self
    where
        F: Fn(FetchRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<::hiera::FetchResponse, ::hiera::FetchError>> + Send + 'static,
    {
        Self::from_store(source, Arc::new(FetchStore::new(fetch)))
    }

    /// Creates a loader that routes CPU parsing work through `spawner`
    /// instead of running it inline on whatever thread polls this loader's
    /// futures.
    ///
    /// This is entirely opt-in: [`Self::open`] behaves identically to before
    /// [`Spawner`] existed by defaulting to [`::hiera::InlineSpawner`].
    pub fn open_with_spawner<F, Fut>(source: Source, fetch: F, spawner: Arc<dyn Spawner>) -> Self
    where
        F: Fn(FetchRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<::hiera::FetchResponse, ::hiera::FetchError>> + Send + 'static,
    {
        Self::from_store_with_spawner(source, Arc::new(FetchStore::new(fetch)), spawner)
    }

    /// Creates a loader over a first-class physical I3S resource store.
    ///
    /// Construction performs no I/O. Applications provide a
    /// [`SceneLayerStore`] for package enumeration or a caller-owned
    /// [`hiera::Fetch`] callback through [`Self::open`].
    pub fn from_store(source: Source, store: Arc<dyn SceneLayerStore>) -> Self {
        Self::from_store_with_spawner(source, store, Arc::new(::hiera::InlineSpawner))
    }

    /// Creates a loader over a first-class physical I3S resource store that
    /// routes CPU parsing work through `spawner` instead of running it
    /// inline on whatever thread polls this loader's futures.
    ///
    /// This is entirely opt-in: [`Self::from_store`] behaves identically to
    /// before [`Spawner`] existed by defaulting to [`::hiera::InlineSpawner`].
    pub fn from_store_with_spawner(
        source: Source,
        store: Arc<dyn SceneLayerStore>,
        spawner: Arc<dyn Spawner>,
    ) -> Self {
        Self {
            source,
            preference: GeometryEncodingPreference::default(),
            fetcher: Arc::new(Fetcher::new(store, spawner)),
            layer: Arc::new(Mutex::new(None)),
        }
    }

    /// Returns the configured source naming policy.
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Creates a loader for another layer address using this loader's
    /// transport and geometry-selection configuration.
    ///
    /// This is used when a Building reader descends into a geometry-bearing
    /// sublayer. The new loader has a distinct layer/page cache because the
    /// sublayer has its own document and hierarchy, while authentication,
    /// retry, proxy, and store behavior remain caller-owned and shared.
    pub(crate) fn at_source(&self, source: Source) -> Self {
        Self {
            source,
            preference: self.preference,
            fetcher: Arc::clone(&self.fetcher),
            layer: Arc::new(Mutex::new(None)),
        }
    }

    /// Resolves and reads one logical I3S resource through this loader's
    /// configured source and caller-supplied transport.
    ///
    /// This is the I3S-specific capability consumed by [`crate::SceneLayerReader`].
    pub(crate) fn read_location(
        &self,
        location: crate::ResourceLocation,
    ) -> ::hiera::BoxFuture<::hiera::Bytes, Error> {
        self.read_uri(location.resolve(&self.source))
    }

    /// Reads one already-resolved I3S resource through the caller-owned
    /// transport. Legacy node documents use relative href graphs, so their
    /// resolved addresses cannot be represented by numeric resource IDs.
    pub(crate) fn read_uri(&self, uri: String) -> ::hiera::BoxFuture<::hiera::Bytes, Error> {
        let fetcher = Arc::clone(&self.fetcher);
        Box::pin(async move { fetcher.bytes(uri).await })
    }

    /// Lists package entries when the configured store supports enumeration.
    pub(crate) fn package_entries(
        &self,
    ) -> ::hiera::BoxFuture<Option<Vec<crate::ResourcePath>>, Error> {
        let fetcher = Arc::clone(&self.fetcher);
        Box::pin(async move { fetcher.entries().await })
    }

    /// Selects the compressed geometry buffer where one exists.
    ///
    /// Only useful to a caller with its own Draco decoder;
    /// [`crate::UncompressedGeometryCodec`] rejects compressed buffers.
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
///
/// Parsing (probing `layerType`, then the profile-specific typed parse) runs
/// through `fetcher`'s [`Spawner`] via [`parse_layer_document`], so
/// applications may opt into running this CPU work off whatever thread polls
/// the loader's future.
async fn fetch_layer(
    fetcher: &Fetcher,
    source: &Source,
    preference: GeometryEncodingPreference,
) -> Result<LayerState, Error> {
    let uri = source.layer();
    let bytes = fetcher.bytes(uri.clone()).await?;
    let source = source.clone();
    ::hiera::spawn(&*fetcher.spawner, move || {
        parse_layer_document(&bytes, &uri, &source, preference)
    })
    .await
    .map_err(Error::spawn)?
}

/// Synchronously parses a layer document's bytes into a [`LayerState`].
///
/// `layerType` decides the profile. Every profile declares it, and the
/// allowed values are disjoint across profiles, so one probe suffices.
fn parse_layer_document(
    bytes: &::hiera::Bytes,
    uri: &str,
    source: &Source,
    preference: GeometryEncodingPreference,
) -> Result<LayerState, Error> {
    let probe: LayerTypeProbe =
        serde_json::from_slice(bytes).map_err(|error| Error::parse(uri, error.to_string()))?;
    let parse = |error: serde_json::Error| Error::parse(uri, error.to_string());

    match probe.layer_type.as_deref() {
        Some("PointCloud") => {
            let layer: pcsl::LayerPcsl = serde_json::from_slice(bytes).map_err(parse)?;
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
                ProfileLoader::PointCloudPaged { nodes_per_page },
                preference,
            ))
        }

        Some("Point") => {
            let layer: psl::SceneLayerInfoPsl = serde_json::from_slice(bytes).map_err(parse)?;
            let layer_id = layer.id;
            // "For legacy purposes, this property is called pointNodePages"
            // -- 3DSceneLayer.psl.md. It is an ordinary nodePageDefinition.
            let profile = paged_profile(layer.point_node_pages.as_ref(), layer_id, true)?;
            Ok(LayerState::new(
                source.clone().with_layer_id(layer_id),
                LayerDocument::Point(Box::new(layer)),
                profile,
                preference,
            ))
        }

        Some("Building") => {
            let layer: bld::LayerBld = serde_json::from_slice(bytes).map_err(parse)?;
            let layer_id = layer.id;
            // A building layer owns no nodes; its content is reached by
            // descending into each non-`group` sublayer.
            let mut ids = Vec::new();
            collect_geometry_sublayers(&layer.sublayers, &mut ids);
            Ok(LayerState::new(
                source.clone().with_layer_id(layer_id),
                LayerDocument::Building(Box::new(layer)),
                ProfileLoader::Sublayers { ids },
                preference,
            ))
        }

        _ => {
            let layer: SceneLayerInfo = serde_json::from_slice(bytes).map_err(parse)?;
            let layer_id = layer.id;
            let profile = match layer.node_pages.as_ref() {
                Some(pages) => paged_profile(Some(pages), layer_id, false)?,
                // Absent `nodePages` is not an error: it is how a 1.6 layer
                // says its tree is a graph of `3dNodeIndexDocument`s.
                None => ProfileLoader::Legacy,
            };
            Ok(LayerState::new(
                source.clone().with_layer_id(layer_id),
                LayerDocument::Mesh(Box::new(layer)),
                profile,
                preference,
            ))
        }
    }
}

/// Builds a profile delegate from a shared node-page definition.
fn paged_profile(
    pages: Option<&cmn::NodePageDefinition>,
    layer_id: u32,
    point: bool,
) -> Result<ProfileLoader, Error> {
    let Some(pages) = pages else {
        return Ok(ProfileLoader::Legacy);
    };
    let nodes_per_page = u64::from(pages.nodes_per_page);
    if nodes_per_page == 0 {
        return Err(Error::InvalidNodesPerPage {
            layer_id,
            nodes_per_page: 0,
        });
    }
    let root = u64::from(pages.root_index.unwrap_or(0));
    Ok(if point {
        ProfileLoader::Point {
            nodes_per_page,
            root,
        }
    } else {
        ProfileLoader::MeshPyramids {
            nodes_per_page,
            root,
        }
    })
}

/// Collects the ids of every sublayer that holds geometry.
///
/// `group` sublayers are organisational only - they carry no resources of
/// their own - but they nest, so the tree is walked rather than scanned:
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
    type ItemId = String;
    type ContentRef = ContentRef;
    type ItemInfo = NodeInfo;
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

    fn item_id(&self, item: &Self::Item) -> Self::ItemId {
        let source = &item.layer.source;
        match &item.address {
            NodeAddress::Paged { index } => {
                format!("{}#node:{index}", source.layer())
            }
            NodeAddress::Legacy { uri } => format!("{uri}#legacy"),
            NodeAddress::Sublayers => format!("{}#sublayers", source.layer()),
        }
    }

    fn describe(&self, item: Self::Item) -> DescribeFuture<Self::ItemInfo, Self::Error> {
        let fetcher = self.fetcher.clone();
        Box::pin(async move { describe_node(&fetcher, item).await })
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

/// Returns the node page containing `index`, fetching it once.
async fn page_for(
    fetcher: &Fetcher,
    layer: &Arc<LayerState>,
    index: u64,
) -> Result<(Arc<NodePage>, u64, usize), Error> {
    let (nodes_per_page, format) = layer.profile.page_format().ok_or(Error::NoNodePages {
        layer_id: layer.source.layer_id(),
    })?;
    let page_id = page_id_of(index, nodes_per_page);
    let offset = page_offset_of(index, nodes_per_page);

    if let Some(page) = layer.pages.lock().unwrap().get(&page_id).cloned() {
        return Ok((page, page_id, offset));
    }

    let uri = layer.source.node_page(page_id);
    let page = match format {
        PageFormat::PointCloud => {
            NodePage::PointCloud(fetcher.json::<pcsl::NodePageDefinitionPcsl>(uri).await?)
        }
        PageFormat::Common => NodePage::Mesh(fetcher.json::<cmn::NodePage>(uri).await?),
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
/// layer document and yields its root - after which every existing
/// traversal path applies unchanged.
async fn expand_sublayers(
    fetcher: &Fetcher,
    layer: Arc<LayerState>,
    depth: usize,
    preference: GeometryEncodingPreference,
) -> Result<Expansion<NodeRef, ContentRef>, Error> {
    let Some(ids) = layer.profile.sublayer_ids() else {
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

/// Resolves compact selection facts for a retained node handle.
async fn describe_node(fetcher: &Fetcher, item: NodeRef) -> Result<NodeInfo, Error> {
    let depth = item.depth;
    match item.address {
        NodeAddress::Paged { index } => {
            let (page, page_id, offset) = page_for(fetcher, &item.layer, index).await?;
            match page.as_ref() {
                NodePage::Mesh(page) => {
                    let node = page.nodes.get(offset).ok_or(Error::MissingNode {
                        node_index: index,
                        page_id,
                    })?;
                    Ok(NodeInfo {
                        bounds: NodeBounds::OrientedBox(node.obb.clone()),
                        lod_threshold: node.lod_threshold,
                        depth,
                    })
                }
                NodePage::PointCloud(page) => {
                    let node = page.nodes.get(offset).ok_or(Error::MissingNode {
                        node_index: index,
                        page_id,
                    })?;
                    Ok(NodeInfo {
                        bounds: NodeBounds::OrientedBox(node.obb.clone()),
                        lod_threshold: node.lod_threshold,
                        depth,
                    })
                }
            }
        }
        NodeAddress::Legacy { uri } => {
            let document = legacy_node(fetcher, &item.layer, &uri).await?;
            Ok(NodeInfo {
                bounds: document
                    .obb
                    .clone()
                    .map(NodeBounds::OrientedBox)
                    .unwrap_or(NodeBounds::Sphere(document.mbs)),
                lod_threshold: document
                    .lod_selection
                    .first()
                    .map(|selection| selection.max_error),
                depth,
            })
        }
        NodeAddress::Sublayers => Ok(NodeInfo {
            bounds: NodeBounds::Unbounded,
            lod_threshold: None,
            depth,
        }),
    }
}

async fn legacy_node(
    fetcher: &Fetcher,
    layer: &Arc<LayerState>,
    uri: &str,
) -> Result<Arc<cmn::NodeIndexDocument>, Error> {
    if let Some(document) = layer.legacy_nodes.lock().unwrap().get(uri).cloned() {
        return Ok(document);
    }
    let document = Arc::new(
        fetcher
            .json::<cmn::NodeIndexDocument>(uri.to_owned())
            .await?,
    );
    layer
        .legacy_nodes
        .lock()
        .unwrap()
        .insert(uri.to_owned(), Arc::clone(&document));
    Ok(document)
}

/// Expands a legacy 1.6 `3dNodeIndexDocument`.
async fn expand_legacy(
    fetcher: &Fetcher,
    layer: Arc<LayerState>,
    uri: String,
    depth: usize,
) -> Result<Expansion<NodeRef, ContentRef>, Error> {
    let document = legacy_node(fetcher, &layer, &uri).await?;

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

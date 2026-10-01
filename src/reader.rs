//! Typed, demand-driven reading of one I3S scene layer.

use std::sync::Arc;

use hiera::{Bytes, DescribeFuture, ExpandFuture, LoadFuture, Loader, RootFuture};
use serde::de::DeserializeOwned;

use crate::{
    ContentRef, Error, LoadedContent, NodeInfo, NodeRef, ResourceLocation, ResourcePath,
    SceneLayerLoader, Source, bld, cmn, pcsl, psl,
};

/// A parsed scene-layer document in its native profile representation.
#[derive(Debug, Clone)]
pub enum SceneLayerDocument {
    /// A 3D Object or Integrated Mesh layer.
    Mesh(Box<cmn::SceneLayerInfo>),
    /// A point scene layer.
    Point(Box<psl::SceneLayerInfoPsl>),
    /// A point-cloud scene layer.
    PointCloud(Box<pcsl::LayerPcsl>),
    /// A building scene layer.
    Building(Box<bld::LayerBld>),
}

/// A parsed node-page document in its native profile representation.
#[derive(Debug, Clone)]
pub enum NodePageDocument {
    /// A mesh or point node page.
    Mesh(cmn::NodePage),
    /// A point-cloud node page.
    PointCloud(pcsl::NodePageDefinitionPcsl),
}

/// Typed access to the resources belonging to a single logical scene layer.
#[derive(Clone)]
pub struct SceneLayerReader {
    loader: Arc<SceneLayerLoader>,
}

impl SceneLayerReader {
    /// Creates a typed reader over a shared I3S hierarchy loader.
    pub fn new(loader: Arc<SceneLayerLoader>) -> Self {
        Self { loader }
    }

    /// Returns the I3S loader this reader composes with.
    pub fn loader(&self) -> &SceneLayerLoader {
        &self.loader
    }

    /// Returns the source naming policy used by this reader.
    pub fn source(&self) -> &Source {
        self.loader.source()
    }

    /// Creates a typed reader for one geometry-bearing Building sublayer.
    ///
    /// The returned reader shares the configured store and transport with this
    /// reader, including application-supplied authentication and caching, but
    /// resolves documents and resources below the selected sublayer.
    pub fn sublayer(&self, id: u32) -> Self {
        Self::new(Arc::new(self.loader.at_source(self.source().sublayer(id))))
    }

    /// Reads raw bytes for a logical resource.
    pub async fn bytes(&self, location: ResourceLocation) -> Result<Bytes, Error> {
        self.loader().read_location(location).await
    }

    /// Lists all physical package entries when its store is enumerable.
    ///
    /// Returns `None` for caller-provided remote transports, which cannot
    /// safely enumerate an arbitrary REST or static HTTP package.
    pub async fn package_entries(&self) -> Result<Option<Vec<ResourcePath>>, Error> {
        self.loader().package_entries().await
    }

    /// Reads and deserializes a JSON resource.
    pub async fn json<T: DeserializeOwned>(&self, location: ResourceLocation) -> Result<T, Error> {
        let name = format!("{location:?}");
        let bytes = self.bytes(location).await?;
        serde_json::from_slice(&bytes).map_err(|source| Error::parse(name, source.to_string()))
    }

    /// Reads the root layer document and dispatches to its native profile.
    pub async fn layer(&self) -> Result<SceneLayerDocument, Error> {
        let name = "layer document".to_string();
        let bytes = self.bytes(ResourceLocation::Layer).await?;
        let probe: LayerTypeProbe = serde_json::from_slice(&bytes)
            .map_err(|source| Error::parse(name.clone(), source.to_string()))?;
        let parse = |source: serde_json::Error| Error::parse(name.clone(), source.to_string());
        match probe.layer_type.as_deref() {
            Some("PointCloud") => serde_json::from_slice(&bytes)
                .map(|value| SceneLayerDocument::PointCloud(Box::new(value)))
                .map_err(parse),
            Some("Point") => serde_json::from_slice(&bytes)
                .map(|value| SceneLayerDocument::Point(Box::new(value)))
                .map_err(parse),
            Some("Building") => serde_json::from_slice(&bytes)
                .map(|value| SceneLayerDocument::Building(Box::new(value)))
                .map_err(parse),
            _ => serde_json::from_slice(&bytes)
                .map(|value| SceneLayerDocument::Mesh(Box::new(value)))
                .map_err(parse),
        }
    }

    /// Reads a node page using the supplied profile kind.
    pub async fn node_page(&self, id: u64, point_cloud: bool) -> Result<NodePageDocument, Error> {
        if point_cloud {
            self.json(ResourceLocation::NodePage(id))
                .await
                .map(NodePageDocument::PointCloud)
        } else {
            self.json(ResourceLocation::NodePage(id))
                .await
                .map(NodePageDocument::Mesh)
        }
    }

    /// Reads package metadata when the source publishes `metadata.json`.
    pub async fn metadata(&self) -> Result<cmn::Metadata, Error> {
        self.json(ResourceLocation::Metadata).await
    }

    /// Reads a 1.6 `3dNodeIndexDocument` addressed by its numeric node ID.
    pub async fn legacy_node(&self, id: u32) -> Result<cmn::NodeIndexDocument, Error> {
        self.json(ResourceLocation::LegacyNode(id)).await
    }

    /// Reads a 1.6 node document at its already-resolved href address.
    ///
    /// Legacy layers use a relative href graph and node document names are
    /// not required to be numeric. Callers walking the retained loader tree
    /// can pass its node URI directly, preserving the configured transport.
    pub async fn legacy_node_uri(
        &self,
        uri: impl Into<String>,
    ) -> Result<cmn::NodeIndexDocument, Error> {
        let uri = uri.into();
        let bytes = self.loader().read_uri(uri.clone()).await?;
        serde_json::from_slice(&bytes).map_err(|source| Error::parse(uri, source.to_string()))
    }

    /// Reads a 1.6 shared-resource document addressed by its node ID.
    pub async fn shared_resource(&self, id: u32) -> Result<cmn::SharedResources, Error> {
        self.json(ResourceLocation::SharedResource(id)).await
    }

    /// Reads a typed per-attribute or summary statistics document.
    ///
    /// Statistics schemas vary by I3S profile, so callers choose the
    /// generated document type matching their layer.
    pub async fn statistics<T: DeserializeOwned>(
        &self,
        location: ResourceLocation,
    ) -> Result<T, Error> {
        match location {
            ResourceLocation::Statistics { .. } | ResourceLocation::StatisticsSummary => {
                self.json(location).await
            }
            _ => Err(Error::InvalidRewrite {
                message: "statistics reads require a statistics resource location".into(),
            }),
        }
    }
}

impl Loader for SceneLayerReader {
    type Item = NodeRef;
    type ItemId = String;
    type ContentRef = ContentRef;
    type ItemInfo = NodeInfo;
    type Content = LoadedContent;
    type Error = Error;

    fn root(&self) -> RootFuture<Self::Item, Self::Error> {
        self.loader.root()
    }

    fn item_id(&self, item: &Self::Item) -> Self::ItemId {
        self.loader.item_id(item)
    }

    fn describe(&self, item: Self::Item) -> DescribeFuture<Self::ItemInfo, Self::Error> {
        self.loader.describe(item)
    }

    fn expand(&self, item: Self::Item) -> ExpandFuture<Self::Item, Self::ContentRef, Self::Error> {
        self.loader.expand(item)
    }

    fn load(&self, content: Self::ContentRef) -> LoadFuture<Self::Content, Self::Error> {
        self.loader.load(content)
    }
}

#[derive(serde::Deserialize)]
struct LayerTypeProbe {
    #[serde(rename = "layerType")]
    layer_type: Option<String>,
}

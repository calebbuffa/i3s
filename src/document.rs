//! Scene-layer and node-page documents in their native profile representation.

use crate::{Error, bld, cmn, pcsl, psl};

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

impl SceneLayerDocument {
    /// Parses one complete I3S scene-layer document.
    ///
    /// An application provider may retain the returned value in a
    /// format-specific shared cache. The `uri` is used only for error context.
    pub fn parse(bytes: &[u8], uri: &str) -> Result<Self, Error> {
        parse_scene_layer_document(bytes, uri)
    }
}

/// A parsed node-page document in its native profile representation.
#[derive(Debug, Clone)]
pub enum NodePageDocument {
    /// A mesh or point node page.
    Mesh(cmn::NodePage),
    /// A point-cloud node page.
    PointCloud(pcsl::NodePageDefinitionPcsl),
}

impl NodePageDocument {
    /// Parses one mesh/point or point-cloud node-page document.
    ///
    /// The profile selects the schema; the parser never guesses based on the
    /// document's fields.
    pub fn parse(bytes: &[u8], uri: &str, point_cloud: bool) -> Result<Self, Error> {
        parse_node_page_document(bytes, uri, point_cloud)
    }
}

pub(crate) fn parse_scene_layer_document(
    bytes: &[u8],
    uri: &str,
) -> Result<SceneLayerDocument, Error> {
    #[derive(serde::Deserialize)]
    struct Probe {
        #[serde(rename = "layerType")]
        layer_type: Option<String>,
    }

    let probe: Probe =
        serde_json::from_slice(bytes).map_err(|error| Error::parse(uri, error.to_string()))?;
    let parse = |error: serde_json::Error| Error::parse(uri, error.to_string());
    match probe.layer_type.as_deref() {
        Some("PointCloud") => serde_json::from_slice(bytes)
            .map(|layer| SceneLayerDocument::PointCloud(Box::new(layer)))
            .map_err(parse),
        Some("Point") => serde_json::from_slice(bytes)
            .map(|layer| SceneLayerDocument::Point(Box::new(layer)))
            .map_err(parse),
        Some("Building") => serde_json::from_slice(bytes)
            .map(|layer| SceneLayerDocument::Building(Box::new(layer)))
            .map_err(parse),
        _ => serde_json::from_slice(bytes)
            .map(|layer| SceneLayerDocument::Mesh(Box::new(layer)))
            .map_err(parse),
    }
}

pub(crate) fn parse_node_page_document(
    bytes: &[u8],
    uri: &str,
    point_cloud: bool,
) -> Result<NodePageDocument, Error> {
    if point_cloud {
        serde_json::from_slice(bytes)
            .map(NodePageDocument::PointCloud)
            .map_err(|source| Error::parse(uri, source.to_string()))
    } else {
        serde_json::from_slice(bytes)
            .map(NodePageDocument::Mesh)
            .map_err(|source| Error::parse(uri, source.to_string()))
    }
}

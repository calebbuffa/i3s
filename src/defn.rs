use serde::{Deserialize, Serialize};

use crate::attr::{AttributeField, AttributeStorageInfo};
use crate::crs::{ElevationInfo, Extent, HeightModelInfo, SpatialReference};
use crate::geom::{DefaultGeometrySchema, GeometryDefinition};
use crate::node::NodePageDefinition;
use crate::options::{
    Capabilities, LayerType, NormalReferenceFrame, Profile, ResourcePattern,
};
use crate::visual::{MaterialDefinition, TextureSetDefinition};

/// Store
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Store {
    pub id: String,
    pub profile: Profile,
    pub version: String,
    #[serde(rename = "resourcePattern")]
    pub resource_pattern: Vec<ResourcePattern>,
    #[serde(rename = "rootNode", default)]
    pub root_node: Option<String>,
    pub extent: Vec<f64>,
    #[serde(rename = "indexCRS")]
    pub index_crs: String,
    #[serde(rename = "vertexCRS")]
    pub vertex_crs: String,
    #[serde(rename = "normalReferenceFrame", default)]
    pub normal_reference_frame: Option<NormalReferenceFrame>,
    #[serde(rename = "lodType")]
    pub lod_type: String,
    #[serde(rename = "defaultGeometrySchema", default)]
    pub default_geometry_schema: Option<DefaultGeometrySchema>,
    #[serde(rename = "lodModel", default)]
    pub lod_model: String,
}

/// SceneDefinition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneDefinition {
    pub id: i32,
    pub name: String,
    #[serde(rename = "spatialReference")]
    pub spatial_reference: SpatialReference,
    #[serde(rename = "layerType")]
    pub layer_type: LayerType,
    pub store: Store,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub capabilities: Option<Vec<Capabilities>>,
    #[serde(default)]
    pub href: Option<String>,
    #[serde(rename = "heightModelInfo", default)]
    pub height_model: Option<HeightModelInfo>,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "copyrightText", default)]
    pub copyright_text: Option<String>,
    #[serde(rename = "zFactor", default)]
    pub z_factor: Option<f64>,
    #[serde(rename = "elevationInfo", default)]
    pub elevation: Option<ElevationInfo>,
    #[serde(default)]
    pub fields: Option<Vec<AttributeField>>,
    #[serde(rename = "attributeStorageInfo", default)]
    pub attribute_storage: Option<Vec<AttributeStorageInfo>>,
    #[serde(rename = "statisticsInfo", default)]
    pub statistics: Option<Vec<serde_json::Value>>,
    #[serde(rename = "nodePages", default)]
    pub node_pages: Option<NodePageDefinition>,
    #[serde(rename = "materialDefinitions", default)]
    pub material_definitions: Option<Vec<MaterialDefinition>>,
    #[serde(rename = "textureSetDefinitions", default)]
    pub texture_set_definitions: Option<Vec<TextureSetDefinition>>,
    #[serde(rename = "geometryDefinitions", default)]
    pub geometry_definitions: Option<Vec<GeometryDefinition>>,
    #[serde(rename = "fullExtent", default)]
    pub full_extent: Option<Extent>,
}

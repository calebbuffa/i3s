#[allow(unused_imports)]
use super::*;
///Objects of this type extend vestedGeometryParams and use one texture and one material. They can be used with aggregated LoD geometries. Component objects provide information on parts of the geometry they belong to, specifically with which material and texture to render them.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleComponentParams {
    ///The ID of the component, only unique within the Geometry.
    pub id: u32,
    ///URL - I3S Pointer reference to the material definition in this node's shared resource, from its root element. If present, used for the entire geometry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<Box<str>>,
    ///UUID of the material, as defined in the shared resources bundle, to use for rendering this component.
    #[serde(
        rename = "materialID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub material_id: Option<u32>,
    ///Optional ID of a texture atlas region which to use with the texture to render this component.
    #[serde(
        rename = "regionID",
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub region_id: Vec<u32>,
    ///URL - I3S Pointer reference to the material definition in this node's shared resource, from its root element. If present, used for the entire geometry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texture: Option<Box<str>>,
    ///Optional ID of the texture, as defined in shared resources, to use with the material to render this component.
    #[serde(
        rename = "textureID",
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub texture_id: Vec<u32>,
}

#[allow(unused_imports)]
use super::*;
///The vertexAttribute object describes valid properties for a single vertex.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VertexAttribute {
    ///The color attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<GeometryAttribute>,
    ///The vertex normal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal: Option<GeometryAttribute>,
    ///The vertex position.
    pub position: GeometryAttribute,
    ///The region attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<GeometryAttribute>,
    ///The first set of UV coordinates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv0: Option<GeometryAttribute>,
}

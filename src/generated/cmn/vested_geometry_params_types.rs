#[allow(unused_imports)]
use super::*;
///This object extends geometryParams and is the abstract parent object for all concrete ('vested') geometryParams objects that directly contain a geometry definition, either as an arrayBufferView or as an embedded geometry.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VestedGeometryParams {
    ///A list of Face Attributes, such as indices to build faces, and their definitions. While there are standard keywords such as position, uv0..uv9, normal and color, this is an open, extendable list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<GeometryAttribute>,
    ///Declares the typology of embedded geometry attributes or those in a geometry resources. When 'Indexed', the indices (faces) must also be declared. Possible values are: `PerAttributeArray`InterleavedArray`Indexed`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topology: Option<VestedGeometryParamsTopology>,
    ///The primitive type of the geometry defined through a vestedGeometryParams object. One of {*triangles*, lines, points}.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<str>>,
    ///A list of Vertex Attributes, such as Position, Normals, UV coordinates, and their definitions. While there are standard keywords such as position, uv0..uv9, normal and color, this is an open, extendable list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertex_attributes: Option<VertexAttribute>,
}

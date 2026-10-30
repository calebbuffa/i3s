#[allow(unused_imports)]
use super::*;
///The defaultGeometry schema is used in stores where all arrayBufferView geometry declarations use the same pattern for face and vertex elements. This schema reduces redundancies of arrayBufferView geometry declarations in a store and reuses the geometryAttribute type from featureData. Only valueType and valuesPerElement are required. # Geometry buffer |fieldName|type|description| ----|------------|----| |vertexCount|UINT32|Number of vertices| |featureCount|UINT32|Number of features.| |position|Float32\[3*vertex count\]|Vertex x,y,z positions.| |normal|Float32\[3*vertex count\]|Normals x,y,z vectors.| |uv0|Float32\[2*vertex count\]|Texture coordinates.| |color|UInt8\[4*vertex count|RGBA colors. |id|UInt64\[feature count\]|Feature IDs.| |faceRange|UInt32[2*feature count|Inclusive [range](../1.7/geometryFaceRange.cmn.md) of the mesh triangles belonging to each feature in the featureID array.| |region|UINT16\[4*vertex count\]|UV [region](../1.7/geometryUVRegion.cmn.md) for repeated textures.|
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultGeometrySchema {
    ///Declaration of the indices into vertex attributes that define faces in the geometry, such as position, normals or texture coordinates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<VertexAttribute>,
    ///Provides the order of the keys in featureAttributes, if present.
    pub feature_attribute_order: Vec<Box<str>>,
    ///Declaration of the attributes per feature in the geometry, such as feature ID or face range.
    pub feature_attributes: FeatureAttribute,
    ///Low-level default geometry type. If defined, all geometries in the store are expected to have this type. Must be: `triangles`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geometry_type: Option<GeometryType>,
    ///Defines header fields in the geometry resources of this store that precede the vertex (and index) data.
    pub header: Vec<HeaderAttribute>,
    ///Defines the ordering of the vertex Attributes.
    pub ordering: Vec<Box<str>>,
    ///Declares the topology of embedded geometry attributes. When 'Indexed', the indices must also be declared in the geometry schema ('faces') and precede the vertexAttribute data. Possible values are: `PerAttributeArray`Indexed`: When Indexed, the indices must also be declared in the geometry schema (faces) and precede the vertexAttribute data.
    pub topology: DefaultGeometrySchemaTopology,
    ///Declaration of the attributes per vertex in the geometry, such as position, normals or texture coordinates.
    pub vertex_attributes: VertexAttribute,
}

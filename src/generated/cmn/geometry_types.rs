#[allow(unused_imports)]
use super::*;
///This is the common container class for all types of geometry definitions used in I3S.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    ///Unique ID of the geometry in this store.
    pub id: u32,
    ///The parameters for a geometry, as an Embedded GeometryParams object, an ArrayBufferView, a GeometryReference object, or a SharedResourceReference object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<GeometryParams>,
    ///3D (4x4) transformation matrix expressed as a linear array of 16 values. Used for methods such as translation, scaling, and rotation.
    #[serde(default)]
    pub transformation: [f64; 16],
    ///The type denotes whether the following geometry is defined by using array buffer views (ArrayBufferView), as an internal reference (GeometryReference), as a reference to a shared Resource (SharedResourceReference) or embedded (Embedded).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<str>>,
}
///Each geometryAttribute object is an accessor, i.e. a view, into an array buffer. There are two types of geometryAttributes - vertexAttributes and faceAttributes. The vertexAttributes describe valid properties for a single vertex, and faceAttributes describe faces and other structures by providing a set of indices. For example, the faces.position index attribute is used to define which vertex positions make up a face.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryAttribute {
    ///The starting byte position where the required bytes begin. Only used with the Geometry **arrayBufferView**.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<u32>,
    ///The element type, from {UInt8, UInt16, Int16, Int32, Int64 or Float32, Float64}.
    pub value_type: GeometryAttributeValueType,
    ///The short number of values need to make a valid element (such as 3 for a xyz position).
    pub values_per_element: u32,
}
///Mesh Geometry Description **Important**: The order of the vertex attributes in the buffer is **fixed** to simplify binary parsing: ` position normal uv0 uv1 color uvRegion featureId faceRange ` or ` compressedAttributes ` **Important:** - Attribute that are present are stored continuously in the corresponding geometry buffers. - All vertex attributes ( **except** `compressedAttributes`) have a fixed size that may be computed as: `#component * sizeof( type ) * {# of vertices or #features}` where `#component` is the number of components such as `position`,`normal`, etc. Furthermore, `type` is the datatype of the variable used and `sizeof` returns the size of the datatype in bytes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryBuffer {
    ///The colors attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<GeometryColor>,
    ///Compressed attributes. **Cannot** be combined with any other attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compressed_attributes: Option<CompressedAttributes>,
    ///Face range for a feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_range: Option<GeometryFaceRange>,
    ///FeatureId attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<GeometryFeatureId>,
    ///Face/vertex normal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal: Option<GeometryNormal>,
    ///The number of bytes to skip from the beginning of the binary buffer. Useful to describe 'legacy' buffer that have a header. Default=`0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    ///Vertex positions relative to oriented-bounding-box center.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<GeometryPosition>,
    ///First set of UV coordinates. Only applies to textured mesh.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv0: Option<GeometryUv>,
    ///UV regions, used for repeated textures in texture atlases.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv_region: Option<GeometryUvRegion>,
}
///The color vertex attribute. Assumed to be Standard RGB (sRGB space). sRGB is a color space that defines a range of colors that can be displayed on screen on in print. It is the most widely used color space and is supported by most operating systems, software programs, monitors, and printers.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryColor {
    ///Must be: `per-vertex`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<GeometryColorBinding>,
    ///Number of colors. Must be `1` (opaque grayscale: `{R,R,R,255}`),`3`(opaque color `{R,G,B,255}`) or `4` ( transparent color `{R,G,B,A}`).
    pub component: u32,
    ///Encoding of the vertex attribute. Must be: `normalized`: Default. Assumes 8-bit unsigned color per channel \[0,255\] -> \[0,1\].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<GeometryColorEncoding>,
    ///The color channel values. Must be: `UInt8`
    pub r#type: GeometryColorType,
}
///The geometry definitions used in I3S version 1.7.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryDefinition {
    ///Array of geometry representation(s) for this class of meshes. When multiple representations are listed, Clients should select the most compact they support (e.g. Draco compressed mesh). For compatibility reasons, _uncompressed_ geometry buffer is always required and must be first (i.e. `geometryBuffers[0]`), so array length must be 1 or 2
    pub geometry_buffers: Vec<GeometryBuffer>,
    ///Defines the topology type of the mesh. Must be: `triangle`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topology: Option<GeometryDefinitionTopology>,
}
///`faceRange` is an inclusive range of faces of the geometry that belongs to a specific feature. For each feature, `faceRange` indicates its first and last triangles as a pair of integer indices in the face list. **Notes**: - [`featureID`](geometryFeatureID.cmn.md) attribute is required - This attributes is only supported when topology is `triangle` - Vertices in the geometry buffer must be grouped by `feature_id` - for _un-indexed triangle meshes_, `vertex_index = face_index * 3 ` **Example** ![Thematic 3D Object Scene Layer without textures](../../docs/img/faceRange.png) _Mesh with 2 features._ ![Thematic 3D Object Scene Layer without textures](../../docs/img/faceRance_Triangles.png) _Grouped vertices in the geometry buffer._
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryFaceRange {
    ///Must be: `per-feature`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<GeometryFaceRangeBinding>,
    ///Pair of indices marking first and last triangles for a feature.
    pub component: u32,
    ///Must be: `none`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<GeometryFaceRangeEncoding>,
    ///Data type for the index rangeMust be: `UInt32`
    pub r#type: GeometryFaceRangeType,
}
///FeatureID attribute helps to identify a part of a mesh belonging to a particular GIS `feature`. This ID may be used to query additional information from a `FeatureService`. For example, if a 3D Object scene layer has a building with ID 1 all triangles in the faceRange for this feature will belong to this feature_id.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryFeatureId {
    ///Must be: `per-feature`: Default for `geometryBuffer.featureId`. One `feature_id` per feature. **Requirement**: a) [`FaceRange`](geometryFaceRange.cmn.md) attribute must be **present** to map features-to-faces and vertices must _be grouped by feature_. **OR** b) [`compressedAttribute.attributes`](compressedAttributes.cmn.md) has `feature-index`. Important: a) and b) are mutually exclusive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<GeometryFeatureIdBinding>,
    ///must be 1
    pub component: u32,
    ///Must be: `none`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<GeometryFeatureIdEncoding>,
    ///A feature integer ID. Possible values are: `UInt16`UInt32`UInt64`
    pub r#type: GeometryFeatureIdType,
}
///Normal attribute. Defines the normals of the geometry.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryNormal {
    ///Must be: `per-vertex`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<GeometryNormalBinding>,
    ///Number of coordinates per vertex position. Must be 3.
    pub component: u32,
    ///EncodingMust be: `none`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<GeometryNormalEncoding>,
    ///Must be: `Float32`
    pub r#type: GeometryNormalType,
}
///The abstract parent object for all geometryParams classes (geometryReferenceParams, vestedGeometryParamas, singleComponentParams). It does not have properties of its own.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryParams {}
///Position vertex attribute. Relative to the center of oriented-bounded box of the node.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryPosition {
    ///Must be: `per-vertex`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<GeometryPositionBinding>,
    ///Number of coordinates per vertex position. Must be 3.
    pub component: u32,
    ///Encoding. Must be: `none`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<GeometryPositionEncoding>,
    ///Vertex positions relative to Oriented-bounding-box center. Must be: `Float32`
    pub r#type: GeometryPositionType,
}
///Instead of owning a geometry exclusively, a feature can reference part of a geometry defined for the node. This allows to pre-aggregate geometries for many features. In this case, geometryReferenceParams must be used. This allows for a single geometry to be shared(referenced) by multiple features.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryReferenceParams {
    ///Inclusive range of faces in this geometry that belongs to this feature.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub face_range: Vec<u32>,
    ///In-document absolute reference to full geometry definition (Embedded or ArrayBufferView) using the I3S json pointer syntax. For example, /geometryData/1. See [OGC I3S Specification](https://docs.opengeospatial.org/cs/17-014r5/17-014r5.html#28) for more info.
    pub href: Box<str>,
    ///True if this geometry participates in an LoD tree. Always true in mesh-pyramids profile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod_geometry: Option<bool>,
    ///The type denotes whether the following geometry is defined by using array buffer views (arrayBufferView), as an internal reference (geometryReference), as a reference to a shared Resource (sharedResourceReference) or embedded (Embedded).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<str>>,
}
///Defines the texture coordinates of the geometry.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryUv {
    ///Must be: `per-vertex`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<GeometryUvBinding>,
    ///Number of texture coordinates. Must be 2.
    pub component: u32,
    ///Must be: `none`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<GeometryUvEncoding>,
    ///Must be: `Float32`
    pub r#type: GeometryUvType,
}
///UV region for repeated textures. UV regions are required to properly wrap UV coordinates of repeated-texture in texture atlases. The texture must be written in the atlas with extra border texels to reduce texture sampling artifacts. UV regions are defined as a four-component array per vertex : \[u_min, v_min, u_max, v_max \], where each component is in the range \[0,1\] encoded using `normalized UInt16`. UV could be "wrapped" in the shader like the following: ` hlsl // UV for this texel is uv in [0, n] uv = frac(uv) * (region.zw - region.xy) + region.xy; `
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryUvRegion {
    ///bindingPossible values are: `per-vertex`: default`per-uvregion`: Only valid in conjonction with [`compressedAttributes`](compressedAttributes.cmn.md) when `uvRegionIndex` attribute is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<GeometryUvRegionBinding>,
    ///The `default =4`, must be 4.
    pub component: u32,
    ///EncodingMust be: `normalized`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<GeometryUvRegionEncoding>,
    ///Color channel values. Must be: `UInt16`
    pub r#type: GeometryUvRegionType,
}

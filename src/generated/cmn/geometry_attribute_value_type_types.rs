#[allow(unused_imports)]
use super::*;
///The element type, from {UInt8, UInt16, Int16, Int32, Int64 or Float32, Float64}.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryAttributeValueType {
    #[serde(rename = "UInt8")]
    UInt8,
    #[serde(rename = "UInt16")]
    UInt16,
    #[serde(rename = "Int16")]
    Int16,
    #[serde(rename = "Int32")]
    Int32,
    #[serde(rename = "Int64")]
    Int64,
    #[serde(rename = "Float32")]
    Float32,
    #[serde(rename = "Float64")]
    Float64,
}
impl Default for GeometryAttributeValueType {
    fn default() -> Self {
        Self::UInt8
    }
}
///Must be: `per-vertex`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryColorBinding {
    #[serde(rename = "per-vertex")]
    PerVertex,
}
impl Default for GeometryColorBinding {
    fn default() -> Self {
        Self::PerVertex
    }
}
///Encoding of the vertex attribute. Must be: `normalized`: Default. Assumes 8-bit unsigned color per channel \[0,255\] -> \[0,1\].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryColorEncoding {
    #[serde(rename = "normalized")]
    Normalized,
}
impl Default for GeometryColorEncoding {
    fn default() -> Self {
        Self::Normalized
    }
}
///The color channel values. Must be: `UInt8`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryColorType {
    #[serde(rename = "UInt8")]
    UInt8,
}
impl Default for GeometryColorType {
    fn default() -> Self {
        Self::UInt8
    }
}
///Defines the topology type of the mesh. Must be: `triangle`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryDefinitionTopology {
    #[serde(rename = "triangle")]
    Triangle,
}
impl Default for GeometryDefinitionTopology {
    fn default() -> Self {
        Self::Triangle
    }
}
///Must be: `per-feature`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryFaceRangeBinding {
    #[serde(rename = "per-feature")]
    PerFeature,
}
impl Default for GeometryFaceRangeBinding {
    fn default() -> Self {
        Self::PerFeature
    }
}
///Must be: `none`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryFaceRangeEncoding {
    #[serde(rename = "none")]
    None,
}
impl Default for GeometryFaceRangeEncoding {
    fn default() -> Self {
        Self::None
    }
}
///Data type for the index rangeMust be: `UInt32`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryFaceRangeType {
    #[serde(rename = "UInt32")]
    UInt32,
}
impl Default for GeometryFaceRangeType {
    fn default() -> Self {
        Self::UInt32
    }
}
///Must be: `per-feature`: Default for `geometryBuffer.featureId`. One `feature_id` per feature. **Requirement**: a) [`FaceRange`](geometryFaceRange.cmn.md) attribute must be **present** to map features-to-faces and vertices must _be grouped by feature_. **OR** b) [`compressedAttribute.attributes`](compressedAttributes.cmn.md) has `feature-index`. Important: a) and b) are mutually exclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryFeatureIdBinding {
    #[serde(rename = "per-feature")]
    PerFeature,
}
impl Default for GeometryFeatureIdBinding {
    fn default() -> Self {
        Self::PerFeature
    }
}
///Must be: `none`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryFeatureIdEncoding {
    #[serde(rename = "none")]
    None,
}
impl Default for GeometryFeatureIdEncoding {
    fn default() -> Self {
        Self::None
    }
}
///A feature integer ID. Possible values are: `UInt16`UInt32`UInt64`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryFeatureIdType {
    #[serde(rename = "UInt16")]
    UInt16,
    #[serde(rename = "UInt32")]
    UInt32,
    #[serde(rename = "UInt64")]
    UInt64,
}
impl Default for GeometryFeatureIdType {
    fn default() -> Self {
        Self::UInt16
    }
}
///Must be: `per-vertex`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryNormalBinding {
    #[serde(rename = "per-vertex")]
    PerVertex,
}
impl Default for GeometryNormalBinding {
    fn default() -> Self {
        Self::PerVertex
    }
}
///EncodingMust be: `none`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryNormalEncoding {
    #[serde(rename = "none")]
    None,
}
impl Default for GeometryNormalEncoding {
    fn default() -> Self {
        Self::None
    }
}
///Must be: `Float32`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryNormalType {
    #[serde(rename = "Float32")]
    Float32,
}
impl Default for GeometryNormalType {
    fn default() -> Self {
        Self::Float32
    }
}
///Must be: `per-vertex`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryPositionBinding {
    #[serde(rename = "per-vertex")]
    PerVertex,
}
impl Default for GeometryPositionBinding {
    fn default() -> Self {
        Self::PerVertex
    }
}
///Encoding. Must be: `none`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryPositionEncoding {
    #[serde(rename = "none")]
    None,
}
impl Default for GeometryPositionEncoding {
    fn default() -> Self {
        Self::None
    }
}
///Vertex positions relative to Oriented-bounding-box center. Must be: `Float32`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryPositionType {
    #[serde(rename = "Float32")]
    Float32,
}
impl Default for GeometryPositionType {
    fn default() -> Self {
        Self::Float32
    }
}
///Low-level default geometry type. If defined, all geometries in the store are expected to have this type. Must be: `triangles`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryType {
    #[serde(rename = "triangles")]
    Triangles,
}
impl Default for GeometryType {
    fn default() -> Self {
        Self::Triangles
    }
}
///Must be: `per-vertex`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryUvBinding {
    #[serde(rename = "per-vertex")]
    PerVertex,
}
impl Default for GeometryUvBinding {
    fn default() -> Self {
        Self::PerVertex
    }
}
///Must be: `none`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryUvEncoding {
    #[serde(rename = "none")]
    None,
}
impl Default for GeometryUvEncoding {
    fn default() -> Self {
        Self::None
    }
}
///bindingPossible values are: `per-vertex`: default`per-uvregion`: Only valid in conjonction with [`compressedAttributes`](compressedAttributes.cmn.md) when `uvRegionIndex` attribute is present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryUvRegionBinding {
    #[serde(rename = "per-vertex")]
    PerVertex,
    #[serde(rename = "per-uvregion")]
    PerUvregion,
}
impl Default for GeometryUvRegionBinding {
    fn default() -> Self {
        Self::PerVertex
    }
}
///EncodingMust be: `normalized`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryUvRegionEncoding {
    #[serde(rename = "normalized")]
    Normalized,
}
impl Default for GeometryUvRegionEncoding {
    fn default() -> Self {
        Self::Normalized
    }
}
///Color channel values. Must be: `UInt16`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryUvRegionType {
    #[serde(rename = "UInt16")]
    UInt16,
}
impl Default for GeometryUvRegionType {
    fn default() -> Self {
        Self::UInt16
    }
}
///Must be: `Float32`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeometryUvType {
    #[serde(rename = "Float32")]
    Float32,
}
impl Default for GeometryUvType {
    fn default() -> Self {
        Self::Float32
    }
}

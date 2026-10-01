//!Auto-generated i3s types. Do not edit manually.
#![allow(missing_docs)]
#![allow(dead_code)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::approx_constant)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::empty_docs)]
use serde::{Deserialize, Serialize};
/// Whether a collection-typed field should be omitted from output.
///
/// A schema property that is absent deserializes to an empty
/// collection, so writing it back out as `[]` or `{}` would not
/// round-trip.
fn is_empty_collection<T: EmptyCollection>(value: &T) -> bool {
    value.is_empty_collection()
}
/// Implemented by the collection types that generated fields use.
trait EmptyCollection {
    /// Whether this collection has no entries.
    fn is_empty_collection(&self) -> bool;
}
impl<T> EmptyCollection for Vec<T> {
    fn is_empty_collection(&self) -> bool {
        self.is_empty()
    }
}
impl<T> EmptyCollection for Box<[T]> {
    fn is_empty_collection(&self) -> bool {
        self.is_empty()
    }
}
impl<K, V> EmptyCollection for std::collections::HashMap<K, V> {
    fn is_empty_collection(&self) -> bool {
        self.is_empty()
    }
}
/// Deserializes a collection, treating `null` as absent.
///
/// A schema that declares an array or object and does not require it
/// is routinely satisfied by producers writing an explicit `null`
/// rather than omitting the key. `#[serde(default)]` only covers the
/// omitted case, so without this a conforming document is rejected.
fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}
pub mod cmn {
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Attributes {
        #[serde(rename = "position")]
        Position,
        #[serde(rename = "normal")]
        Normal,
        #[serde(rename = "uv0")]
        Uv0,
        #[serde(rename = "color")]
        Color,
        #[serde(rename = "uv-region")]
        UvRegion,
        #[serde(rename = "feature-index")]
        FeatureIndex,
    }
    impl Default for Attributes {
        fn default() -> Self {
            Self::Position
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Capabilities {
        #[serde(rename = "View")]
        View,
        #[serde(rename = "Query")]
        Query,
        #[serde(rename = "Edit")]
        Edit,
        #[serde(rename = "Extract")]
        Extract,
    }
    impl Default for Capabilities {
        fn default() -> Self {
            Self::View
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Ordering {
        #[serde(rename = "attributeByteCounts")]
        AttributeByteCounts,
        #[serde(rename = "attributeValues")]
        AttributeValues,
        #[serde(rename = "ObjectIds")]
        ObjectIds,
    }
    impl Default for Ordering {
        fn default() -> Self {
            Self::AttributeByteCounts
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ResourcePattern {
        #[serde(rename = "3dNodeIndexDocument")]
        ThreeDNodeIndexDocument,
        #[serde(rename = "SharedResource")]
        SharedResource,
        #[serde(rename = "featureData")]
        FeatureData,
        #[serde(rename = "Geometry")]
        Geometry,
        #[serde(rename = "Texture")]
        Texture,
        #[serde(rename = "Attributes")]
        Attributes,
    }
    impl Default for ResourcePattern {
        fn default() -> Self {
            Self::ThreeDNodeIndexDocument
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TextureDefinitionInfoWrap {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "repeat")]
        Repeat,
        #[serde(rename = "mirror")]
        Mirror,
    }
    impl Default for TextureDefinitionInfoWrap {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TextureWrap {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "repeat")]
        Repeat,
        #[serde(rename = "mirror")]
        Mirror,
    }
    impl Default for TextureWrap {
        fn default() -> Self {
            Self::None
        }
    }
    ///A feature integer ID. Possible values are: `UInt16`UInt32`UInt64`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    ///Color channel values. Must be: `UInt16`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryUvRegionType {
        #[serde(rename = "UInt16")]
        UInt16,
    }
    impl Default for GeometryUvRegionType {
        fn default() -> Self {
            Self::UInt16
        }
    }
    ///Data type for the index rangeMust be: `UInt32`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryFaceRangeType {
        #[serde(rename = "UInt32")]
        UInt32,
    }
    impl Default for GeometryFaceRangeType {
        fn default() -> Self {
            Self::UInt32
        }
    }
    ///Declares the topology of embedded geometry attributes. When 'Indexed', the indices must also be declared in the geometry schema ('faces') and precede the vertexAttribute data. Possible values are: `PerAttributeArray`Indexed`: When Indexed, the indices must also be declared in the geometry schema (faces) and precede the vertexAttribute data.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DefaultGeometrySchemaTopology {
        #[serde(rename = "PerAttributeArray")]
        PerAttributeArray,
        #[serde(rename = "Indexed")]
        Indexed,
    }
    impl Default for DefaultGeometrySchemaTopology {
        fn default() -> Self {
            Self::PerAttributeArray
        }
    }
    ///Declares the typology of embedded geometry attributes or those in a geometry resources. When 'Indexed', the indices (faces) must also be declared. Possible values are: `PerAttributeArray`InterleavedArray`Indexed`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum VestedGeometryParamsTopology {
        #[serde(rename = "PerAttributeArray")]
        PerAttributeArray,
        #[serde(rename = "InterleavedArray")]
        InterleavedArray,
        #[serde(rename = "Indexed")]
        Indexed,
    }
    impl Default for VestedGeometryParamsTopology {
        fn default() -> Self {
            Self::PerAttributeArray
        }
    }
    ///Defines the meaning of `nodes[].lodThreshold` for this layer. Possible values are: `maxScreenThreshold`: A per-node value for the maximum area of the projected bounding volume on screen in pixel. `maxScreenThresholdSQ`: A per-node value for the maximum area of the projected bounding volume on screen in pixel squared. 3D Viewers may implement **look-angle dependent** node switching by comparing this metric with the area of the 2D outline of the oriented-bounding box (OBB) on screen. ( see [_"Fast Projected Area Computation for Three-Dimensional Bounding Boxes", Dieter Schmalstieg and Robert F. Tobler_](https://pdfs.semanticscholar.org/1f59/8266e387cf367702d16acf5a4e02cc72cb99.pdf) for an efficient algorithm). If a **look-angle independent** LOD switching is desired, viewers may use the area of minimum bounding-sphere (MBS) of the node if available or the MBS of the OBB otherwise. Note: `maxScreenThresholdSQ` may be related to `maxScreenThreshold` as follow: `maxScreenThresholdSQ = PI * 0.25 * maxScreenThreshold * maxScreenThreshold`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum NodePageDefinitionLodSelectionMetricType {
        #[serde(rename = "maxScreenThreshold")]
        MaxScreenThreshold,
        #[serde(rename = "maxScreenThresholdSQ")]
        MaxScreenThresholdSq,
    }
    impl Default for NodePageDefinitionLodSelectionMetricType {
        fn default() -> Self {
            Self::MaxScreenThreshold
        }
    }
    ///Defines the meaning of the alpha-channel/alpha-mask. Possible values are: `opaque`: The rendered output is fully opaque and any alpha value is ignored. `mask`: The rendered output is either fully opaque or fully transparent depending on the alpha value and the specified alpha cutoff value. This mode is used to simulate geometry such as tree leaves or wire fences. `blend`: The rendered output is combined with the background using the normal painting operation (i.e. the Porter and Duff over operator).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AlphaMode {
        #[serde(rename = "opaque")]
        Opaque,
        #[serde(rename = "mask")]
        Mask,
        #[serde(rename = "blend")]
        Blend,
    }
    impl Default for AlphaMode {
        fn default() -> Self {
            Self::Opaque
        }
    }
    ///Defines the topology type of the mesh. Must be: `triangle`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryDefinitionTopology {
        #[serde(rename = "triangle")]
        Triangle,
    }
    impl Default for GeometryDefinitionTopology {
        fn default() -> Self {
            Self::Triangle
        }
    }
    ///Defines the value type. Possible values are: `Int8`UInt8`Int16`UInt16`Int32`UInt32`Float32`Float64`String`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum HeaderValueType {
        #[serde(rename = "Int8")]
        Int8,
        #[serde(rename = "UInt8")]
        UInt8,
        #[serde(rename = "Int16")]
        Int16,
        #[serde(rename = "UInt16")]
        UInt16,
        #[serde(rename = "Int32")]
        Int32,
        #[serde(rename = "UInt32")]
        UInt32,
        #[serde(rename = "Float32")]
        Float32,
        #[serde(rename = "Float64")]
        Float64,
        #[serde(rename = "String")]
        String,
    }
    impl Default for HeaderValueType {
        fn default() -> Self {
            Self::Int8
        }
    }
    ///Deprecated in 1.7. Optional field to indicate the [LoD switching](lodSelection.cmn.md) mode. Possible values are: `node-switching`: A parent node is substituted for its children nodes when its lod threshold is exceeded. This implies that: parent and children are never shown at the same time. The bounding volumne of the parent has to enclose the features of all grandchildren. Nodes have a single parent, except the root node that have no parent. `none`: No switching model.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LodModel {
        #[serde(rename = "node-switching")]
        NodeSwitching,
        #[serde(rename = "none")]
        None,
    }
    impl Default for LodModel {
        fn default() -> Self {
            Self::NodeSwitching
        }
    }
    ///Deprecated in 1.7. Optional field to indicate which LoD generation scheme is used in this store. Possible values are: `MeshPyramid`: Used for integrated mesh and 3D scene layer. `AutoThinning`: Use for point scene layer. `Clustering`: Fill in which profile types are using this lodType`Generalizing`: Fill in which profile types are using this lodType
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LodType {
        #[serde(rename = "MeshPyramid")]
        MeshPyramid,
        #[serde(rename = "AutoThinning")]
        AutoThinning,
        #[serde(rename = "Clustering")]
        Clustering,
        #[serde(rename = "Generalizing")]
        Generalizing,
    }
    impl Default for LodType {
        fn default() -> Self {
            Self::MeshPyramid
        }
    }
    ///Describes the coordinate reference frame used for storing normals. Although not required, it is recommended to re-compute the normal component of the binary geometry buffer if this property is not present. Possible values are: `east-north-up`: Normals are stored in a node local reference frame defined by the easting, northing and up directions at the MBS center. It is only valid for geographic (WGS84) vertexCRS.`earth-centered`: Normals are stored in a global earth-centered, earth-fixed ([ECEF](https://en.wikipedia.org/wiki/ECEF)) reference frame. It is only valid for geographic vertexCRS. Earth centered can be directly used for global view rendering, thus is an optimal option for meshes in non-projected geographic coordinate system. `vertex-reference-frame`: Normals are stored in the same reference frame as vertices. It is only valid for projected vertexCRS.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum NormalReferenceFrame {
        #[serde(rename = "east-north-up")]
        EastNorthUp,
        #[serde(rename = "earth-centered")]
        EarthCentered,
        #[serde(rename = "vertex-reference-frame")]
        VertexReferenceFrame,
    }
    impl Default for NormalReferenceFrame {
        fn default() -> Self {
            Self::EastNorthUp
        }
    }
    ///Encoding method for the time value. DateTime attribute string formatting must comply with [ECMA-ISO 8601](ECMA_ISO8601.md). Must be: `ECMA_ISO8601`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TimeEncoding {
        #[serde(rename = "ECMA_ISO8601")]
        EcmaIso8601,
    }
    impl Default for TimeEncoding {
        fn default() -> Self {
            Self::EcmaIso8601
        }
    }
    ///Encoding method for the value. Possible values are: `count`: Should always be present and indicates the count of features in the attribute storage. `attributeValuesByteCount`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Property {
        #[serde(rename = "count")]
        Count,
        #[serde(rename = "attributeValuesByteCount")]
        AttributeValuesByteCount,
    }
    impl Default for Property {
        fn default() -> Self {
            Self::Count
        }
    }
    ///Encoding of the vertex attribute. Must be: `normalized`: Default. Assumes 8-bit unsigned color per channel [0,255] -> [0,1].
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryColorEncoding {
        #[serde(rename = "normalized")]
        Normalized,
    }
    impl Default for GeometryColorEncoding {
        fn default() -> Self {
            Self::Normalized
        }
    }
    ///Encoding. Must be: `none`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryPositionEncoding {
        #[serde(rename = "none")]
        None,
    }
    impl Default for GeometryPositionEncoding {
        fn default() -> Self {
            Self::None
        }
    }
    ///EncodingMust be: `none`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryNormalEncoding {
        #[serde(rename = "none")]
        None,
    }
    impl Default for GeometryNormalEncoding {
        fn default() -> Self {
            Self::None
        }
    }
    ///EncodingMust be: `normalized`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryUvRegionEncoding {
        #[serde(rename = "normalized")]
        Normalized,
    }
    impl Default for GeometryUvRegionEncoding {
        fn default() -> Self {
            Self::Normalized
        }
    }
    ///Indicates channels description. Possible values are: `rgb`rgba`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TextureChannels {
        #[serde(rename = "rgb")]
        Rgb,
        #[serde(rename = "rgba")]
        Rgba,
    }
    impl Default for TextureChannels {
        fn default() -> Self {
            Self::Rgb
        }
    }
    ///Indicates channels description. Possible values are: `rgb`rgba`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TextureDefinitionInfoChannels {
        #[serde(rename = "rgb")]
        Rgb,
        #[serde(rename = "rgba")]
        Rgba,
    }
    impl Default for TextureDefinitionInfoChannels {
        fn default() -> Self {
            Self::Rgb
        }
    }
    ///Indicates the material type, chosen from the supported values. Possible values are: `standard`water`billboard`leafcard`reference`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum MaterialDefinitionInfoType {
        #[serde(rename = "standard")]
        Standard,
        #[serde(rename = "water")]
        Water,
        #[serde(rename = "billboard")]
        Billboard,
        #[serde(rename = "leafcard")]
        Leafcard,
        #[serde(rename = "reference")]
        Reference,
    }
    impl Default for MaterialDefinitionInfoType {
        fn default() -> Self {
            Self::Standard
        }
    }
    ///Low-level default geometry type. If defined, all geometries in the store are expected to have this type. Must be: `triangles`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryType {
        #[serde(rename = "triangles")]
        Triangles,
    }
    impl Default for GeometryType {
        fn default() -> Self {
            Self::Triangles
        }
    }
    ///Merge policy for the domain. Not used by Scene Layers. Possible values are: `esriMPTDefaultValue`esriMPTSumValues`esriMPTAreaWeighted`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum MergePolicy {
        #[serde(rename = "esriMPTDefaultValue")]
        EsriMptDefaultValue,
        #[serde(rename = "esriMPTSumValues")]
        EsriMptSumValues,
        #[serde(rename = "esriMPTAreaWeighted")]
        EsriMptAreaWeighted,
    }
    impl Default for MergePolicy {
        fn default() -> Self {
            Self::EsriMptDefaultValue
        }
    }
    ///Must be: `Float32`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryNormalType {
        #[serde(rename = "Float32")]
        Float32,
    }
    impl Default for GeometryNormalType {
        fn default() -> Self {
            Self::Float32
        }
    }
    ///Must be: `Float32`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryUvType {
        #[serde(rename = "Float32")]
        Float32,
    }
    impl Default for GeometryUvType {
        fn default() -> Self {
            Self::Float32
        }
    }
    ///Must be: `draco`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum CompressedAttributesEncoding {
        #[serde(rename = "draco")]
        Draco,
    }
    impl Default for CompressedAttributesEncoding {
        fn default() -> Self {
            Self::Draco
        }
    }
    ///Must be: `none`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryFaceRangeEncoding {
        #[serde(rename = "none")]
        None,
    }
    impl Default for GeometryFaceRangeEncoding {
        fn default() -> Self {
            Self::None
        }
    }
    ///Must be: `none`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryFeatureIdEncoding {
        #[serde(rename = "none")]
        None,
    }
    impl Default for GeometryFeatureIdEncoding {
        fn default() -> Self {
            Self::None
        }
    }
    ///Must be: `none`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryUvEncoding {
        #[serde(rename = "none")]
        None,
    }
    impl Default for GeometryUvEncoding {
        fn default() -> Self {
            Self::None
        }
    }
    ///Must be: `per-feature`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryFaceRangeBinding {
        #[serde(rename = "per-feature")]
        PerFeature,
    }
    impl Default for GeometryFaceRangeBinding {
        fn default() -> Self {
            Self::PerFeature
        }
    }
    ///Must be: `per-feature`: Default for `geometryBuffer.featureId`. One `feature_id` per feature. **Requirement**: a) [`FaceRange`](geometryFaceRange.cmn.md) attribute must be **present** to map features-to-faces and vertices must _be grouped by feature_. **OR** b) [`compressedAttribute.attributes`](compressedAttributes.cmn.md) has `feature-index`. Important: a) and b) are mutually exclusive.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryFeatureIdBinding {
        #[serde(rename = "per-feature")]
        PerFeature,
    }
    impl Default for GeometryFeatureIdBinding {
        fn default() -> Self {
            Self::PerFeature
        }
    }
    ///Must be: `per-vertex`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryColorBinding {
        #[serde(rename = "per-vertex")]
        PerVertex,
    }
    impl Default for GeometryColorBinding {
        fn default() -> Self {
            Self::PerVertex
        }
    }
    ///Must be: `per-vertex`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryNormalBinding {
        #[serde(rename = "per-vertex")]
        PerVertex,
    }
    impl Default for GeometryNormalBinding {
        fn default() -> Self {
            Self::PerVertex
        }
    }
    ///Must be: `per-vertex`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryPositionBinding {
        #[serde(rename = "per-vertex")]
        PerVertex,
    }
    impl Default for GeometryPositionBinding {
        fn default() -> Self {
            Self::PerVertex
        }
    }
    ///Must be: `per-vertex`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryUvBinding {
        #[serde(rename = "per-vertex")]
        PerVertex,
    }
    impl Default for GeometryUvBinding {
        fn default() -> Self {
            Self::PerVertex
        }
    }
    ///Possible values are: `maxScreenThreshold`: A per-node value for the maximum pixel size as measured in screen pixels. This value indicates the upper limit for the screen size of the diameter of the node's minimum bounding sphere (MBS). In other words, the content referenced by this node will qualify to be rendered only when the screen size is below the maximum screen threshold value. Used with the mesh pyramid profile. `maxScreenThresholdSQ`: A per-node value for the maximum area of the projected bounding volume on screen in pixel squared. 3D Viewers may implement **look-angle dependent** node switching by comparing this metric with the area of the 2D outline of the oriented-bounding box (OBB) on screen. ( see [_"Fast Projected Area Computation for Three-Dimensional Bounding Boxes", Dieter Schmalstieg and Robert F. Tobler_](https://pdfs.semanticscholar.org/1f59/8266e387cf367702d16acf5a4e02cc72cb99.pdf) for an efficient algorithm). If a **look-angle independent** LoD switching is desired, viewers may use the area of minimum bounding-sphere (MBS) of the node if available or the MBS of the OBB otherwise. Note: `maxScreenThresholdSQ` may be related to `maxScreenThreshold` as follow: `maxScreenThresholdSQ = PI * 0.25 * maxScreenThreshold * maxScreenThreshold`screenSpaceRelative`: The scale of the node's minimum bounding volume. Used by the point profile. `distanceRangeFromDefaultCamera`: The distance from the surface of the node's minimum bounding volume to the camera. Used by the point profile. `effectiveDensity`: Estimation of the point density covered by the node. Used by the point cloud profile.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum MetricType {
        #[serde(rename = "maxScreenThreshold")]
        MaxScreenThreshold,
        #[serde(rename = "maxScreenThresholdSQ")]
        MaxScreenThresholdSq,
        #[serde(rename = "screenSpaceRelative")]
        ScreenSpaceRelative,
        #[serde(rename = "distanceRangeFromDefaultCamera")]
        DistanceRangeFromDefaultCamera,
        #[serde(rename = "effectiveDensity")]
        EffectiveDensity,
    }
    impl Default for MetricType {
        fn default() -> Self {
            Self::MaxScreenThreshold
        }
    }
    ///Possible values are: `relativeToGround`absoluteHeight`onTheGround`relativeToScene`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Mode {
        #[serde(rename = "relativeToGround")]
        RelativeToGround,
        #[serde(rename = "absoluteHeight")]
        AbsoluteHeight,
        #[serde(rename = "onTheGround")]
        OnTheGround,
        #[serde(rename = "relativeToScene")]
        RelativeToScene,
    }
    impl Default for Mode {
        fn default() -> Self {
            Self::RelativeToGround
        }
    }
    ///Rendering mode. Possible values are: `textured`solid`untextured`wireframe`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum RenderMode {
        #[serde(rename = "textured")]
        Textured,
        #[serde(rename = "solid")]
        Solid,
        #[serde(rename = "untextured")]
        Untextured,
        #[serde(rename = "wireframe")]
        Wireframe,
    }
    impl Default for RenderMode {
        fn default() -> Self {
            Self::Textured
        }
    }
    ///Represents the height model type. Possible values are: `gravity_related_height`ellipsoidal`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum HeightModel {
        #[serde(rename = "gravity_related_height")]
        GravityRelatedHeight,
        #[serde(rename = "ellipsoidal")]
        Ellipsoidal,
    }
    impl Default for HeightModel {
        fn default() -> Self {
            Self::GravityRelatedHeight
        }
    }
    ///Represents the unit of the height. Possible values are: `meter`us-foot`foot`clarke-foot`clarke-yard`clarke-link`sears-yard`sears-foot`sears-chain`benoit-1895-b-chain`indian-yard`indian-1937-yard`gold-coast-foot`sears-1922-truncated-chain`us-inch`us-mile`us-yard`millimeter`decimeter`centimeter`kilometer`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum HeightUnit {
        #[serde(rename = "meter")]
        Meter,
        #[serde(rename = "us-foot")]
        UsFoot,
        #[serde(rename = "foot")]
        Foot,
        #[serde(rename = "clarke-foot")]
        ClarkeFoot,
        #[serde(rename = "clarke-yard")]
        ClarkeYard,
        #[serde(rename = "clarke-link")]
        ClarkeLink,
        #[serde(rename = "sears-yard")]
        SearsYard,
        #[serde(rename = "sears-foot")]
        SearsFoot,
        #[serde(rename = "sears-chain")]
        SearsChain,
        #[serde(rename = "benoit-1895-b-chain")]
        Benoit1895BChain,
        #[serde(rename = "indian-yard")]
        IndianYard,
        #[serde(rename = "indian-1937-yard")]
        Indian1937Yard,
        #[serde(rename = "gold-coast-foot")]
        GoldCoastFoot,
        #[serde(rename = "sears-1922-truncated-chain")]
        Sears1922TruncatedChain,
        #[serde(rename = "us-inch")]
        UsInch,
        #[serde(rename = "us-mile")]
        UsMile,
        #[serde(rename = "us-yard")]
        UsYard,
        #[serde(rename = "millimeter")]
        Millimeter,
        #[serde(rename = "decimeter")]
        Decimeter,
        #[serde(rename = "centimeter")]
        Centimeter,
        #[serde(rename = "kilometer")]
        Kilometer,
    }
    impl Default for HeightUnit {
        fn default() -> Self {
            Self::Meter
        }
    }
    ///Split policy for the domain. Not used by Scene Layers. Possible values are: `esriSPTGeometryRatio`esriSPTDuplicate`esriSPTDefaultValue`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SplitPolicy {
        #[serde(rename = "esriSPTGeometryRatio")]
        EsriSptGeometryRatio,
        #[serde(rename = "esriSPTDuplicate")]
        EsriSptDuplicate,
        #[serde(rename = "esriSPTDefaultValue")]
        EsriSptDefaultValue,
    }
    impl Default for SplitPolicy {
        fn default() -> Self {
            Self::EsriSptGeometryRatio
        }
    }
    ///The color channel values. Must be: `UInt8`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryColorType {
        #[serde(rename = "UInt8")]
        UInt8,
    }
    impl Default for GeometryColorType {
        fn default() -> Self {
            Self::UInt8
        }
    }
    ///The element type of the header property. Possible values are: `UInt8`UInt16`UInt32`UInt64`Int16`Int32`Int64`Float32`Float64`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum HeaderAttributeType {
        #[serde(rename = "UInt8")]
        UInt8,
        #[serde(rename = "UInt16")]
        UInt16,
        #[serde(rename = "UInt32")]
        UInt32,
        #[serde(rename = "UInt64")]
        UInt64,
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
    impl Default for HeaderAttributeType {
        fn default() -> Self {
            Self::UInt8
        }
    }
    ///The element type, from {UInt8, UInt16, Int16, Int32, Int64 or Float32, Float64}.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    ///The field type is the type of attribute field with which the domain can be associated. Possible values are: `esriFieldTypeDate`esriFieldTypeSingle`esriFieldTypeDouble`esriFieldTypeInteger`esriFieldTypeSmallInteger`esriFieldTypeString`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DomainFieldType {
        #[serde(rename = "esriFieldTypeDate")]
        EsriFieldTypeDate,
        #[serde(rename = "esriFieldTypeSingle")]
        EsriFieldTypeSingle,
        #[serde(rename = "esriFieldTypeDouble")]
        EsriFieldTypeDouble,
        #[serde(rename = "esriFieldTypeInteger")]
        EsriFieldTypeInteger,
        #[serde(rename = "esriFieldTypeSmallInteger")]
        EsriFieldTypeSmallInteger,
        #[serde(rename = "esriFieldTypeString")]
        EsriFieldTypeString,
    }
    impl Default for DomainFieldType {
        fn default() -> Self {
            Self::EsriFieldTypeDate
        }
    }
    ///The texture format. Possible values are: `jpg`: JPEG compression. No mipmaps. Please note that alpha channel may have been added after the JPEG stream. This alpha channel is alwasy 8bit and zlib compressed. Last 4 bytes of the entire stream are the 32 bit offset to the beginning of the alpha stream (little-endian).`png`: PNG format, no mipmaps`dds`: The DDS header will specify the type of compression and number of mipmaps. **WARNING:** Only DXT1 (no alpha) and DXT5 (alpha channel) are supported. `ktx-etc2`: Khronos group container for ETC2 compressed texture. Mipmap may be available. Note: KTX (Khronos Texture) is a lightweight file format for OpenGL® textures, designed around how textures are loaded in OpenGL.`ktx2`: Basis Universal Supercompressed GPU Texture.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Format {
        #[serde(rename = "jpg")]
        Jpg,
        #[serde(rename = "png")]
        Png,
        #[serde(rename = "dds")]
        Dds,
        #[serde(rename = "ktx-etc2")]
        KtxEtc2,
        #[serde(rename = "ktx2")]
        Ktx2,
    }
    impl Default for Format {
        fn default() -> Self {
            Self::Jpg
        }
    }
    ///The user-visible layer typePossible values are: `3DObject`IntegratedMesh`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LayerType {
        #[serde(rename = "3DObject")]
        ThreeDObject,
        #[serde(rename = "IntegratedMesh")]
        IntegratedMesh,
    }
    impl Default for LayerType {
        fn default() -> Self {
            Self::ThreeDObject
        }
    }
    ///Type of domainPossible values are: `codedValue`range`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DomainType {
        #[serde(rename = "codedValue")]
        CodedValue,
        #[serde(rename = "range")]
        Range,
    }
    impl Default for DomainType {
        fn default() -> Self {
            Self::CodedValue
        }
    }
    ///Type of the field. Possible values are: `esriFieldTypeDate`esriFieldTypeSingle`esriFieldTypeDouble`esriFieldTypeGUID`esriFieldTypeGlobalID`esriFieldTypeInteger`esriFieldTypeOID`esriFieldTypeSmallInteger`esriFieldTypeString`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum FieldType {
        #[serde(rename = "esriFieldTypeDate")]
        EsriFieldTypeDate,
        #[serde(rename = "esriFieldTypeSingle")]
        EsriFieldTypeSingle,
        #[serde(rename = "esriFieldTypeDouble")]
        EsriFieldTypeDouble,
        #[serde(rename = "esriFieldTypeGUID")]
        EsriFieldTypeGuid,
        #[serde(rename = "esriFieldTypeGlobalID")]
        EsriFieldTypeGlobalId,
        #[serde(rename = "esriFieldTypeInteger")]
        EsriFieldTypeInteger,
        #[serde(rename = "esriFieldTypeOID")]
        EsriFieldTypeOid,
        #[serde(rename = "esriFieldTypeSmallInteger")]
        EsriFieldTypeSmallInteger,
        #[serde(rename = "esriFieldTypeString")]
        EsriFieldTypeString,
    }
    impl Default for FieldType {
        fn default() -> Self {
            Self::EsriFieldTypeDate
        }
    }
    ///Vertex positions relative to Oriented-bounding-box center. Must be: `Float32`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryPositionType {
        #[serde(rename = "Float32")]
        Float32,
    }
    impl Default for GeometryPositionType {
        fn default() -> Self {
            Self::Float32
        }
    }
    ///Winding order is counterclockwise. Possible values are: `none`: Default. **Must** be none if `doubleSided=True`.`front`: Cull front faces (i.e. faces with counter-clockwise winding order).`back`: Cull back faces (i.e. faces with clockwise winding order).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum CullFace {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "front")]
        Front,
        #[serde(rename = "back")]
        Back,
    }
    impl Default for CullFace {
        fn default() -> Self {
            Self::None
        }
    }
    ///bindingPossible values are: `per-vertex`: default`per-uvregion`: Only valid in conjonction with [`compressedAttributes`](compressedAttributes.cmn.md) when `uvRegionIndex` attribute is present.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    ///The attributeStorageInfo object describes the structure of the binary attribute data resource of a layer, which is the same for every node in the layer. The following examples show how different attribute types are represented as a binary buffer. # Examples of attribute resources
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AttributeStorageInfo {
        ///For string types only. Represents the byte count of the string, including the null character.
        #[serde(
            rename = "attributeByteCounts",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attribute_byte_counts: Option<Value>,
        ///Represents the description for value encoding. For example: scalar or vector encoding.
        #[serde(
            rename = "attributeValues",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attribute_values: Option<Value>,
        ///Declares the headers of the binary attribute data.
        pub header: Vec<HeaderValue>,
        ///The unique field identifier key.
        pub key: Box<str>,
        ///The name of the field.
        pub name: Box<str>,
        ///Stores the object-id values of each feature within the node.
        #[serde(rename = "objectIds", default, skip_serializing_if = "Option::is_none")]
        pub object_ids: Option<Value>,
        ///Possible values for each array string: `attributeByteCounts`: Should only be present when working with string data types. `attributeValues`: Should always be present. `ObjectIds`
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub ordering: Vec<Ordering>,
    }
    ///The cachedDrawingInfo object indicates if the *drawingInfo* object is captured as part of the binary scene layer representation. This object is used for the 3D Object and Integrated Mesh scene layer if no [drawingInfo](drawingInfo.cmn.md) is defined.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CachedDrawingInfo {
        ///If true, the drawingInfo is captured as part of the binary scene layer representation.
        pub color: bool,
    }
    ///I3S version 1.7 supports compressing the geometryBuffer of Integrated Mesh and 3D Object Layers using [Draco](https://github.com/google/draco) compression. Draco compression is optimized for compressing and decompressing 3D geometric meshes and point clouds. Draco reduces the size of the geometryBuffer payload, thereby reducing storage size and optimizing transmission rate. All *vertexAttributes* of a Meshpyramids profile can be compressed with Draco. *The ArcGIS platform currently is compatible with version 1.3.5 of [Draco](https://github.com/google/draco/blob/master/README.md#version-135-release).*
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CompressedAttributes {
        ///Possible values for each array string: `position`: `Draco` _double_ meta-data `i3s-scale_x`, `i3s-scale_y`. If present, must be applied to `x` and `y` coordinates to reverse `XY`/`Z` ratio preserving scaling that may have been applied before encoding. (i.e.avoid quantization issue when `XY` is in degrees and `Z` is in meters)`normal`uv0`color`uv-region`: Uses `draco::GeometryAttribute::Type::GENERIC` with type `4xUINT16`. The attribute meta-data key `i3s-attribute-type` *must* be set to `"uv-region"` (string).`feature-index`: Uses `draco::GeometryAttribute::Type::GENERIC` with type `1xUINT32`. The attribute meta-data key `i3s-attribute-type` *must* be set to `"feature-index"` (string). The `feature-ids` values must be stored in the `feature-index` attribute meta-data with `key:"i3s-feature-ids" ` (metata data entry type is array of int32)
        pub attributes: Vec<Attributes>,
        ///Must be: `draco`
        pub encoding: CompressedAttributesEncoding,
    }
    ///The defaultGeometry schema is used in stores where all arrayBufferView geometry declarations use the same pattern for face and vertex elements. This schema reduces redundancies of arrayBufferView geometry declarations in a store and reuses the geometryAttribute type from featureData. Only valueType and valuesPerElement are required. # Geometry buffer |fieldName|type|description| ----|------------|----| |vertexCount|UINT32|Number of vertices| |featureCount|UINT32|Number of features.| |position|Float32[3*vertex count]|Vertex x,y,z positions.| |normal|Float32[3*vertex count]|Normals x,y,z vectors.| |uv0|Float32[2*vertex count]|Texture coordinates.| |color|UInt8[4*vertex count|RGBA colors. |id|UInt64[feature count]|Feature IDs.| |faceRange|UInt32[2*feature count|Inclusive [range](../1.7/geometryFaceRange.cmn.md) of the mesh triangles belonging to each feature in the featureID array.| |region|UINT16[4*vertex count]|UV [region](../1.7/geometryUVRegion.cmn.md) for repeated textures.|
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DefaultGeometrySchema {
        ///Declaration of the indices into vertex attributes that define faces in the geometry, such as position, normals or texture coordinates.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub faces: Option<VertexAttribute>,
        ///Provides the order of the keys in featureAttributes, if present.
        #[serde(rename = "featureAttributeOrder")]
        pub feature_attribute_order: Vec<Box<str>>,
        ///Declaration of the attributes per feature in the geometry, such as feature ID or face range.
        #[serde(rename = "featureAttributes")]
        pub feature_attributes: FeatureAttribute,
        ///Low-level default geometry type. If defined, all geometries in the store are expected to have this type. Must be: `triangles`
        #[serde(
            rename = "geometryType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub geometry_type: Option<GeometryType>,
        ///Defines header fields in the geometry resources of this store that precede the vertex (and index) data.
        pub header: Vec<HeaderAttribute>,
        ///Defines the ordering of the vertex Attributes.
        pub ordering: Vec<Box<str>>,
        ///Declares the topology of embedded geometry attributes. When 'Indexed', the indices must also be declared in the geometry schema ('faces') and precede the vertexAttribute data. Possible values are: `PerAttributeArray`Indexed`: When Indexed, the indices must also be declared in the geometry schema (faces) and precede the vertexAttribute data.
        pub topology: DefaultGeometrySchemaTopology,
        ///Declaration of the attributes per vertex in the geometry, such as position, normals or texture coordinates.
        #[serde(rename = "vertexAttributes")]
        pub vertex_attributes: VertexAttribute,
    }
    ///Attribute domains are rules that describe the legal values of a field type, providing a method for enforcing data integrity. Attribute domains are used to constrain the values allowed in a particular attribute. Using domains helps ensure data integrity by limiting the choice of values for a particular field. Attribute domains can be shared across scene layers like 3D Object scene layers or Building Scene Layers.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Domain {
        ///Range of the domain. Only string types are possible.
        #[serde(
            rename = "codedValues",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub coded_values: Vec<DomainCodedValue>,
        ///Description of the domain
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<Box<str>>,
        ///The field type is the type of attribute field with which the domain can be associated. Possible values are: `esriFieldTypeDate`esriFieldTypeSingle`esriFieldTypeDouble`esriFieldTypeInteger`esriFieldTypeSmallInteger`esriFieldTypeString`
        #[serde(rename = "fieldType", default, skip_serializing_if = "Option::is_none")]
        pub field_type: Option<DomainFieldType>,
        ///Merge policy for the domain. Not used by Scene Layers. Possible values are: `esriMPTDefaultValue`esriMPTSumValues`esriMPTAreaWeighted`
        #[serde(
            rename = "mergePolicy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub merge_policy: Option<MergePolicy>,
        ///Name of the domain. Must be unique per Scene Layer.
        pub name: Box<str>,
        ///Range of the domain. Only numeric types are possible.
        #[serde(default)]
        pub range: [f64; 2],
        ///Split policy for the domain. Not used by Scene Layers. Possible values are: `esriSPTGeometryRatio`esriSPTDuplicate`esriSPTDefaultValue`
        #[serde(
            rename = "splitPolicy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub split_policy: Option<SplitPolicy>,
        ///Type of domainPossible values are: `codedValue`range`
        #[serde(rename = "type")]
        pub r#type: DomainType,
    }
    ///Attribute domains are rules that describe the legal values of a field type, providing a method for enforcing data integrity. Attribute domains are used to constrain the values allowed in any particular attribute. Whenever a domain is associated with an attribute field, only the values within that domain are valid for the field. Using domains helps ensure data integrity by limiting the choice of values for a particular field. The domain code value contains the coded values for a domain as well as an associated description of what that value represents.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DomainCodedValue {
        ///Coded value (i.e. field value).
        pub code: serde_json::Value,
        ///Text representation of the domain value.
        pub name: Box<str>,
    }
    ///The drawingInfo object contains drawing information for a scene layer.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DrawingInfo {
        ///An object defining the symbology for the layer. [See more](https://developers.arcgis.com/web-scene-specification/objects/drawingInfo/) information about supported renderer types in ArcGIS clients.
        pub renderer: serde_json::Value,
        ///Scale symbols for the layer.
        #[serde(
            rename = "scaleSymbols",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub scale_symbols: Option<bool>,
    }
    ///An object defining where a feature is placed within a scene. For example, on the ground or at an absolute height. [See more](https://developers.arcgis.com/web-scene-specification/objects/elevationInfo/) information on elevation in ArcGIS clients.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ElevationInfo {
        ///Possible values are: `relativeToGround`absoluteHeight`onTheGround`relativeToScene`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mode: Option<Mode>,
        ///Offset is always added to the result of the above logic except for onTheGround where offset is ignored.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: Option<f64>,
        ///A string value indicating the unit for the values in elevationInfo
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<Box<str>>,
    }
    ///Declaration of the attributes per feature in the geometry, such as feature ID or face range.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FeatureAttribute {
        ///Describes the face range of the feature attribute.
        #[serde(rename = "faceRange", default, skip_serializing_if = "Option::is_none")]
        pub face_range: Option<Value>,
        ///ID of the feature attribute.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<Value>,
    }
    ///The FeatureData JSON file(s) contain geographical features with a set of attributes, accessors to geometry attributes, and other references to styling or materials. FeatureData is only used by point scene layers. For other scene layer types, such as 3D object scene layer or integrated mesh scene layer, clients read [defaultGeometrySchema](defaultGeometrySchema.cmn.md) to access the geometry buffer.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FeatureData {
        ///The list of GIS attributes the feature has.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub attributes: Option<FeatureAttribute>,
        ///The list of geometries the feature has. A feature always has at least one Geometry.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub geometries: Option<Geometry>,
        ///Feature ID, unique within the Node. If lodType is FeatureTree, the ID must be unique in the store.
        pub id: u32,
        ///The name of the Feature Class this feature belongs to.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub layer: Option<Box<str>>,
        ///An array of six doubles, corresponding to xmin, ymin, zmin, xmax, ymax and zmax of the minimum bounding box of the feature, expressed in the vertexCRS, without offset. The mbb can be used with the Feature's Transform to provide a LOD0 representation without loading the GeometryAttributes.
        #[serde(default)]
        pub mbb: [f64; 6],
        ///An array of three doubles, providing an optional, 'semantic' pivot offset that can be used to e.g. correctly drape tree symbols.
        #[serde(rename = "pivotOffset", default)]
        pub pivot_offset: [f64; 3],
        ///An array of two or three doubles, giving the x,y(,z) (easting/northing/elevation) position of this feature's minimum bounding sphere center, in the vertexCRS.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub position: Vec<f64>,
    }
    ///Declaration of the attributes per feature in the geometry, such as feature ID or face range.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Features {
        #[serde(
            rename = "featureData",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub feature_data: Vec<FeatureData>,
        #[serde(
            rename = "geometryData",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub geometry_data: Vec<Geometry>,
    }
    ///A collection of objects describing each attribute field.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Field {
        ///Alias of the field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<Box<str>>,
        ///Array of domains defined for a field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<Domain>,
        ///Name of the field.
        pub name: Box<str>,
        ///Type of the field. Possible values are: `esriFieldTypeDate`esriFieldTypeSingle`esriFieldTypeDouble`esriFieldTypeGUID`esriFieldTypeGlobalID`esriFieldTypeInteger`esriFieldTypeOID`esriFieldTypeSmallInteger`esriFieldTypeString`
        #[serde(rename = "type")]
        pub r#type: FieldType,
    }
    ///The 3D spatial extent of the object it describes in the given spatial reference. The coordinates of the extent can span across the antimeridian (180th meridian). For example, scene layers in a geographic coordinate system covering New Zealand may have a larger xmin value than xmax value. The fullExtent is used by clients to zoom to a scene layer.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FullExtent {
        ///An object containing the WKID or WKT identifying the spatial reference of the layer's geometry.
        #[serde(
            rename = "spatialReference",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub spatial_reference: Option<SpatialReference>,
        ///The most west x coordinate.
        pub xmax: f64,
        ///The most east x coordinate.
        pub xmin: f64,
        ///The most north y coordinate.
        pub ymax: f64,
        ///The most south y coordinate.
        pub ymin: f64,
        ///The maximum height z coordinate.
        pub zmax: f64,
        ///The minimum height z coordinate.
        pub zmin: f64,
    }
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
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub r#type: Option<Box<str>>,
    }
    ///Each geometryAttribute object is an accessor, i.e. a view, into an array buffer. There are two types of geometryAttributes - vertexAttributes and faceAttributes. The vertexAttributes describe valid properties for a single vertex, and faceAttributes describe faces and other structures by providing a set of indices. For example, the faces.position index attribute is used to define which vertex positions make up a face.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GeometryAttribute {
        ///The starting byte position where the required bytes begin. Only used with the Geometry **arrayBufferView**.
        #[serde(
            rename = "byteOffset",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub byte_offset: Option<u32>,
        ///The element type, from {UInt8, UInt16, Int16, Int32, Int64 or Float32, Float64}.
        #[serde(rename = "valueType")]
        pub value_type: GeometryAttributeValueType,
        ///The short number of values need to make a valid element (such as 3 for a xyz position).
        #[serde(rename = "valuesPerElement")]
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
        #[serde(
            rename = "compressedAttributes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compressed_attributes: Option<CompressedAttributes>,
        ///Face range for a feature.
        #[serde(rename = "faceRange", default, skip_serializing_if = "Option::is_none")]
        pub face_range: Option<GeometryFaceRange>,
        ///FeatureId attribute.
        #[serde(rename = "featureId", default, skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "uvRegion", default, skip_serializing_if = "Option::is_none")]
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
        ///Encoding of the vertex attribute. Must be: `normalized`: Default. Assumes 8-bit unsigned color per channel [0,255] -> [0,1].
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encoding: Option<GeometryColorEncoding>,
        ///The color channel values. Must be: `UInt8`
        #[serde(rename = "type")]
        pub r#type: GeometryColorType,
    }
    ///The geometry definitions used in I3S version 1.7.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GeometryDefinition {
        ///Array of geometry representation(s) for this class of meshes. When multiple representations are listed, Clients should select the most compact they support (e.g. Draco compressed mesh). For compatibility reasons, _uncompressed_ geometry buffer is always required and must be first (i.e. `geometryBuffers[0]`), so array length must be 1 or 2
        #[serde(rename = "geometryBuffers")]
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
        #[serde(rename = "type")]
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
        #[serde(rename = "type")]
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
        #[serde(rename = "type")]
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
        #[serde(rename = "type")]
        pub r#type: GeometryPositionType,
    }
    ///Instead of owning a geometry exclusively, a feature can reference part of a geometry defined for the node. This allows to pre-aggregate geometries for many features. In this case, geometryReferenceParams must be used. This allows for a single geometry to be shared(referenced) by multiple features.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GeometryReferenceParams {
        ///Inclusive range of faces in this geometry that belongs to this feature.
        #[serde(
            rename = "faceRange",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub face_range: Vec<u32>,
        ///In-document absolute reference to full geometry definition (Embedded or ArrayBufferView) using the I3S json pointer syntax. For example, /geometryData/1. See [OGC I3S Specification](https://docs.opengeospatial.org/cs/17-014r5/17-014r5.html#28) for more info.
        pub href: Box<str>,
        ///True if this geometry participates in an LoD tree. Always true in mesh-pyramids profile.
        #[serde(
            rename = "lodGeometry",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub lod_geometry: Option<bool>,
        ///The type denotes whether the following geometry is defined by using array buffer views (arrayBufferView), as an internal reference (geometryReference), as a reference to a shared Resource (sharedResourceReference) or embedded (Embedded).
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "type")]
        pub r#type: GeometryUvType,
    }
    ///UV region for repeated textures. UV regions are required to properly wrap UV coordinates of repeated-texture in texture atlases. The texture must be written in the atlas with extra border texels to reduce texture sampling artifacts. UV regions are defined as a four-component array per vertex : [u_min, v_min, u_max, v_max ], where each component is in the range [0,1] encoded using `normalized UInt16`. UV could be "wrapped" in the shader like the following: ` hlsl // UV for this texel is uv in [0, n] uv = frac(uv) * (region.zw - region.xy) + region.xy; `
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
        #[serde(rename = "type")]
        pub r#type: GeometryUvRegionType,
    }
    ///The header definition provides the name of each field and the value type. Headers to geometry resources must be uniform across any cache and may only contain fixed-width, single element fields.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HeaderAttribute {
        ///The name of the property in the header.
        pub property: Box<str>,
        ///The element type of the header property. Possible values are: `UInt8`UInt16`UInt32`UInt64`Int16`Int32`Int64`Float32`Float64`
        #[serde(rename = "type")]
        pub r#type: HeaderAttributeType,
    }
    ///Value for attributeByteCount, attributeValues and objectIds.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HeaderValue {
        ///Encoding method for the value. Possible values are: `count`: Should always be present and indicates the count of features in the attribute storage. `attributeValuesByteCount`
        pub property: Property,
        ///Defines the value type. Possible values are: `Int8`UInt8`Int16`UInt16`Int32`UInt32`Float32`Float64`String`
        #[serde(rename = "valueType")]
        pub value_type: HeaderValueType,
    }
    ///The I3S standard accommodates declaration of a vertical coordinate system that may either be ellipsoidal or gravity-related. This allows for a diverse range of fields and applications where the definition of elevation/height is important.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HeightModelInfo {
        ///Represents the height model type. Possible values are: `gravity_related_height`ellipsoidal`
        #[serde(
            rename = "heightModel",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub height_model: Option<HeightModel>,
        ///Represents the unit of the height. Possible values are: `meter`us-foot`foot`clarke-foot`clarke-yard`clarke-link`sears-yard`sears-foot`sears-chain`benoit-1895-b-chain`indian-yard`indian-1937-yard`gold-coast-foot`sears-1922-truncated-chain`us-inch`us-mile`us-yard`millimeter`decimeter`centimeter`kilometer`
        #[serde(
            rename = "heightUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub height_unit: Option<HeightUnit>,
        ///Represents the vertical coordinate system.
        #[serde(rename = "vertCRS", default, skip_serializing_if = "Option::is_none")]
        pub vert_crs: Option<Box<str>>,
    }
    ///The bin size may be computed as (max-min)/bin count. Please note that stats.histo.min/max is not equivalent to stats.min/max since values smaller than stats.histo.min and greater than stats.histo.max are counted in the first and last bin respectively. The values stats.min and stats.max may be conservative estimates. The bins would be distributed as follows: `(-inf, stats.min + bin_size], (stats.min + bin_size, stats.min + 2 * bin_size], ... , (stats.min + (bin_count - 1) * bin_size], (stats.min + (bin_count - 1) * bin_size, +inf)`
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Histogram {
        ///Array of binned value counts with up to `n` values, where `n` is the number of bins and **must be less or equal to 256**.
        pub counts: Vec<u32>,
        ///Maximum value (i.e. right bound) of the last bin of the histogram.
        pub maximum: f64,
        ///Minimum value (i.e. left bound) of the first bin of the histogram.
        pub minimum: f64,
    }
    ///An image is a binary resource, containing a single raster that can be used to texture a feature or symbol. An image represents one specific texture LoD. For details on texture organization, please refer to the section on [texture resources](texture.cmn.md).
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Image {
        ///The byte offset of this image's encodings. There is one per encoding, in the same order as the encodings, in the block in which this texture image resides.
        #[serde(
            rename = "byteOffset",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub byte_offset: Vec<u32>,
        ///The href to the image(s), one per encoding, in the same order as the encodings.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub href: Vec<Box<str>>,
        ///A unique ID for each image. Generated using the BuildID function.
        pub id: Box<str>,
        ///The length in bytes of this image's encodings. There is one per encoding, in the same order as the encodings.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub length: Vec<u32>,
        ///The maximum size of a single pixel in world units. This property is used by the client to pick the image to load and render.
        #[serde(
            rename = "pixelInWorldUnits",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub pixel_in_world_units: Option<f64>,
        ///width of this image, in pixels.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
    }
    ///LoD (Level of Detail) selection. A client needs information to determine whether a node's contents are "good enough" to render in the current 3D view under constraints such as resolution, screen size, bandwidth and available memory and target minimum quality goals. Multiple LoD selection metrics can be included. These metrics are used by clients to determine the optimal resource access patterns. Each I3S profile definition provides additional details on LoD Selection.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LodSelection {
        ///Maximum metric value, expressed in the CRS of the vertex coordinates or in reference to other constants such as screen size.
        #[serde(rename = "maxError")]
        pub max_error: f64,
        ///Possible values are: `maxScreenThreshold`: A per-node value for the maximum pixel size as measured in screen pixels. This value indicates the upper limit for the screen size of the diameter of the node's minimum bounding sphere (MBS). In other words, the content referenced by this node will qualify to be rendered only when the screen size is below the maximum screen threshold value. Used with the mesh pyramid profile. `maxScreenThresholdSQ`: A per-node value for the maximum area of the projected bounding volume on screen in pixel squared. 3D Viewers may implement **look-angle dependent** node switching by comparing this metric with the area of the 2D outline of the oriented-bounding box (OBB) on screen. ( see [_"Fast Projected Area Computation for Three-Dimensional Bounding Boxes", Dieter Schmalstieg and Robert F. Tobler_](https://pdfs.semanticscholar.org/1f59/8266e387cf367702d16acf5a4e02cc72cb99.pdf) for an efficient algorithm). If a **look-angle independent** LoD switching is desired, viewers may use the area of minimum bounding-sphere (MBS) of the node if available or the MBS of the OBB otherwise. Note: `maxScreenThresholdSQ` may be related to `maxScreenThreshold` as follow: `maxScreenThresholdSQ = PI * 0.25 * maxScreenThreshold * maxScreenThreshold`screenSpaceRelative`: The scale of the node's minimum bounding volume. Used by the point profile. `distanceRangeFromDefaultCamera`: The distance from the surface of the node's minimum bounding volume to the camera. Used by the point profile. `effectiveDensity`: Estimation of the point density covered by the node. Used by the point cloud profile.
        #[serde(rename = "metricType")]
        pub metric_type: MetricType,
    }
    ///Materials describe how a feature or a set of features is to be rendered, including shading and color. Part of [sharedResource](sharedResource.cmn.md) that is deprecated with 1.7.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MaterialDefinition {
        #[serde(default, flatten)]
        pub additional_properties: std::collections::HashMap<String, MaterialDefinitionInfo>,
    }
    ///Material information describes how a feature or a set of features is to be rendered, including shading and color. The following table provides the set of attributes and parameters for the `type`: `standard` material. Part of [sharedResource](sharedResource.cmn.md) that is deprecated with 1.7.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MaterialDefinitionInfo {
        ///The href that resolves to the shared resource bundle in which the material definition is contained.
        #[serde(rename = "$ref", default, skip_serializing_if = "Option::is_none")]
        pub r#ref: Option<String>,
        ///A name for the material as assigned in the creating application.
        pub name: Box<str>,
        ///Parameter defined for the material.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub params: Option<MaterialParams>,
        ///Indicates the material type, chosen from the supported values. Possible values are: `standard`water`billboard`leafcard`reference`
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub r#type: Option<MaterialDefinitionInfoType>,
    }
    ///The materialDefinitions object in I3S version 1.7 and higher are feature-compatible with [glTF material](https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#materials) but with the following exceptions. I3S material colors properties (baseColorFactor, emissiveFactor etc.) are assumed to be in the same color space as the textures, most commonly sRGB while in glTF they are interpreted as [linear](https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#metallic-roughness-material). glTF has separate definitions for properties like strength for [occlusionTextureInfo](https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/schema/material.occlusionTextureInfo.schema.json) and scale for [normalTextureInfo](https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/schema/material.normalTextureInfo.schema.json). Further I3S has only one [texture definition](materialTexture.cmn.md) with factor that replaces strength and scale.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MaterialDefinitions {
        ///The alpha cutoff value of the material (only applies when alphaMode=`mask`) default = `0.25`. If the alpha value is greater than or equal to the `alphaCutoff` value then it is rendered as fully opaque, otherwise, it is rendered as fully transparent.
        #[serde(
            rename = "alphaCutoff",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub alpha_cutoff: Option<f64>,
        ///Defines the meaning of the alpha-channel/alpha-mask. Possible values are: `opaque`: The rendered output is fully opaque and any alpha value is ignored. `mask`: The rendered output is either fully opaque or fully transparent depending on the alpha value and the specified alpha cutoff value. This mode is used to simulate geometry such as tree leaves or wire fences. `blend`: The rendered output is combined with the background using the normal painting operation (i.e. the Porter and Duff over operator).
        #[serde(rename = "alphaMode", default, skip_serializing_if = "Option::is_none")]
        pub alpha_mode: Option<AlphaMode>,
        ///Winding order is counterclockwise. Possible values are: `none`: Default. **Must** be none if `doubleSided=True`.`front`: Cull front faces (i.e. faces with counter-clockwise winding order).`back`: Cull back faces (i.e. faces with clockwise winding order).
        #[serde(rename = "cullFace", default, skip_serializing_if = "Option::is_none")]
        pub cull_face: Option<CullFace>,
        ///Specifies whether the material is double sided. For lighting, the opposite normals will be used when original normals are facing away from the camera. default=`false`.
        #[serde(
            rename = "doubleSided",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub double_sided: Option<bool>,
        ///The emissive color of the material.
        #[serde(rename = "emissiveFactor", default)]
        pub emissive_factor: [f64; 3],
        ///The emissive texture map. A texture that receives no lighting, so the pixels are shown at full intensity.
        #[serde(
            rename = "emissiveTexture",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub emissive_texture: Option<MaterialTexture>,
        ///The normal texture map. They are a special kind of texture that allow you to add surface detail such as bumps, grooves, and scratches to a model which catch the light as if they are represented by real geometry.
        #[serde(
            rename = "normalTexture",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub normal_texture: Option<MaterialTexture>,
        ///The occlusion texture map. The occlusion map is used to provide information about which areas of the model should receive high or low indirect lighting
        #[serde(
            rename = "occlusionTexture",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub occlusion_texture: Option<MaterialTexture>,
        ///A set of parameter values that are used to define the metallic-roughness material model from Physically-Based Rendering (PBR) methodology. When not specified, all the default values of pbrMetallicRoughness apply.
        #[serde(
            rename = "pbrMetallicRoughness",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub pbr_metallic_roughness: Option<PbrMetallicroughness>,
    }
    ///Parameters describing the material.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MaterialParams {
        ///Ambient color of this material. Ambient color is the color of an object where it is in shadow. This color is what the object reflects when illuminated by ambient light rather than direct light.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub ambient: Vec<f64>,
        ///TRUE if features with this material should cast shadows.
        #[serde(
            rename = "castShadows",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub cast_shadows: Option<bool>,
        ///Indicates the material culling options {back, front, *none*}.
        #[serde(rename = "cullFace", default, skip_serializing_if = "Option::is_none")]
        pub cull_face: Option<Box<str>>,
        ///Diffuse color of this material. Diffuse color is the most instinctive meaning of the color of an object. It is that essential color that the object reveals under pure white light. It is perceived as the color of the object itself rather than a reflection of the light.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub diffuse: Vec<f64>,
        ///TRUE if features with this material should receive shadows
        #[serde(
            rename = "receiveShadows",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub receive_shadows: Option<bool>,
        ///Indicates reflectivity of this material.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reflectivity: Option<f64>,
        ///Rendering mode. Possible values are: `textured`solid`untextured`wireframe`
        #[serde(rename = "renderMode")]
        pub render_mode: RenderMode,
        ///Indicates shininess of this material.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shininess: Option<f64>,
        ///Specular color of this material. Specular color is the color of the light of a specular reflection (specular reflection is the type of reflection that is characteristic of light reflected from a shiny surface).
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub specular: Vec<f64>,
        ///Indicates transparency of this material; 0 = opaque, 1 = fully transparent.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub transparency: Option<f64>,
        ///Indicates whether Vertex Colors also contain a transparency channel. Default is false.
        #[serde(
            rename = "useVertexColorAlpha",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_vertex_color_alpha: Option<bool>,
        ///This flag indicates that the vertex color attribute of the geometry should be used to color the geometry for rendering. If texture is present, the vertex colors are multiplied by this color. e.g. `pixel_color = [interpolated]vertex_color * texel_color`. Default is false.
        #[serde(
            rename = "vertexColors",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub vertex_colors: Option<bool>,
        ///This flag indicates that the geometry has uv region vertex attributes. These are used for adressing subtextures in a texture atlas. The uv coordinates are relative to this subtexture in this case. This is mostly useful for repeated textures in a texture atlas. Default is false.
        #[serde(
            rename = "vertexRegions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub vertex_regions: Option<bool>,
    }
    ///The material texture definition.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MaterialTexture {
        ///The _normal texture_: scalar multiplier applied to each normal vector of the normal texture. For _occlusion texture_,scalar multiplier controlling the amount of occlusion applied. Default=`1`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub factor: Option<f64>,
        ///The set index of texture's TEXCOORD attribute used for texture coordinate mapping. Default is 0. Deprecated.
        #[serde(rename = "texCoord", default, skip_serializing_if = "Option::is_none")]
        pub tex_coord: Option<u32>,
        ///The index in [layer.textureSetDefinitions](3DSceneLayer.cmn.md).
        #[serde(rename = "textureSetDefinitionId")]
        pub texture_set_definition_id: u32,
    }
    ///An array of four doubles, corresponding to x, y, z and radius of the minimum bounding sphere of a node.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Mbs {
        ///The center point of the minimum bounding sphere. An array of four doubles, corresponding to x, y, z and radius of the minimum bounding sphere of a node. For a global scene, i.e. XY coordinate system in WGS1984, the values of the array correspond to longitude in decimal degrees, latitude of in decimal degrees, elevation in meters and radius in meters. For all other CRS, the values of x,y,z and r are in the same unit.
        pub mbs: [f64; 4],
    }
    ///Mesh object. Mesh geometry for a node. Clients have to use the `resource` identifiers written in each node to access the resources. While content creator may choose to match `resource` with the node id this is not required by the I3S specification and clients should not make this assumption.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Mesh {
        ///The attribute set definition.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub attribute: Option<MeshAttribute>,
        ///The geometry definition.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub geometry: Option<MeshGeometry>,
        ///The material definition.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub material: Option<MeshMaterial>,
    }
    ///Mesh attributes for a node.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MeshAttribute {
        ///The resource identifier to be used to locate attribute resources of this mesh. i.e. `layers/0/nodes//attributes/...`
        pub resource: u32,
    }
    ///Mesh geometry for a node.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MeshGeometry {
        ///The index in [layer.geometryDefinitions](geometryDefinition.cmn.md) array
        pub definition: u32,
        ///Number of features for this mesh. Default=`0`. (Must omit or set to `0` if mesh doesn't use `features`.)
        #[serde(
            rename = "featureCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub feature_count: Option<u32>,
        ///The resource locator to be used to query geometry resources: `layers/0/nodes/{this.resource}/geometries/{layer.geometryDefinitions[this.definition].geometryBuffers[0 or 1]}`.
        pub resource: u32,
        ///Number of vertices in the geometry buffer of this mesh for the **umcompressed mesh buffer**. Please note that `Draco` compressed meshes may have less vertices due to de-duplication (actual number of vertices is part of the Draco binary blob). Default=`0`
        #[serde(
            rename = "vertexCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub vertex_count: Option<u32>,
    }
    ///Mesh geometry for a node.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MeshMaterial {
        ///The index in [layer.materialDefinitions](3DSceneLayer.cmn.md) array.
        pub definition: u32,
        ///Resource id for the material textures. i.e: `layers/0/nodes/{material.resource}/textures/{tex_name}`. Is **required** if material declares any textures.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<u32>,
        ///Estimated number of texel for the highest resolution base color texture. i.e. `texture.mip0.width*texture.mip0.height`. Useful to estimate the resource cost of this node and/or texel-resolution based LOD switching. Ignored for un-textured meshes.
        #[serde(
            rename = "texelCountHint",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub texel_count_hint: Option<u32>,
    }
    ///The metadata.json contains information regarding the creation and storing of i3s in SLPK to support clients with i3s service creation and processing of the data.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Metadata {
        ///Total number of nodes in the SLPK.
        #[serde(rename = "nodeCount", default, skip_serializing_if = "Option::is_none")]
        pub node_count: Option<u32>,
    }
    ///The node object.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Node {
        ///index of the children nodes indices.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub children: Vec<u32>,
        ///The index in the node array. May be **different than** material, geometry and attribute `resource` id. See [`mesh`](mesh.cmn.md) for more information.
        pub index: u32,
        ///When to switch LoD. See [`nodepages[i].lodSelectionMetricType`](nodePageDefinition.cmn.md) for more information.
        #[serde(
            rename = "lodThreshold",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub lod_threshold: Option<f64>,
        ///The mesh for this node. **WARNING:** only **SINGLE** mesh is supported at version 1.7 (i.e. `length` **must** be 0 or 1).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mesh: Option<Mesh>,
        ///Oriented bounding box for this node.
        pub obb: Obb,
        ///The index of the parent node in the node array.
        #[serde(
            rename = "parentIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub parent_index: Option<u32>,
    }
    ///The 3dNodeIndexDocument JSON file describes a single index node within a [store](store.cmn.md). The store object describes the exact physical storage of a layer and enables the client to detect when multiple layers are served from the same store. The file includes links to other nodes (e.g. children, sibling, and parent), links to feature data, geometry data, texture data resources, metadata (e.g. metrics used for LoD selection), and spatial extent. The node is the root object in the 3dNodeIndexDocument. There is always exactly one node object in a 3dNodeIndexDocument. Depending on the geometry and LoD model, a node document can be tuned towards being light-weight or heavy-weight. Clients decide which data to retrieve. The bounding volume information for the node, its parent, siblings, and children provide enough data for a simple visualization. For example, the centroids of a bounding volume could be rendered as point features.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NodeIndexDocument {
        ///Resource reference describing a featureData document.
        #[serde(
            rename = "attributeData",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub attribute_data: Vec<Resource>,
        ///Reference to the child nodes of a node.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub children: Vec<NodeReference>,
        ///Creation date of this node in UTC, presented as a string in the format YYYY-MM-DDThh:mm:ss.sTZD, with a fixed 'Z' time zone (see http://www.w3.org/TR/NOTE-datetime).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created: Option<Box<str>>,
        ///Expiration date of this node in UTC, presented as a string in the format YYYY-MM-DDThh:mm:ss.sTZD, with a fixed 'Z' time zone (see http://www.w3.org/TR/NOTE-datetime).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expires: Option<Box<str>>,
        ///Resource reference describing a FeatureData document.
        #[serde(
            rename = "featureData",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub feature_data: Vec<Resource>,
        ///**Deprecated.** A list of summary information on the features present in this node, used for pre-visualisation and LoD switching in featureTree LoD stores.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub features: Vec<Features>,
        ///Resource reference describing a geometry resource.
        #[serde(
            rename = "geometryData",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub geometry_data: Vec<Resource>,
        ///Tree-key ID. A unique identifier of a node within the scene layer. At 1.7 the tree-key is the integer id of the node represented as a string.
        pub id: Box<str>,
        ///Explicit level of this node within the index tree. The lowest level is 0, which is always the root node.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<u32>,
        ///Metrics for LoD selection, to be evaluated by the client. *This property was previously optional which was a documentation error.
        #[serde(rename = "lodSelection")]
        pub lod_selection: Vec<LodSelection>,
        ///The center point of the minimum bounding sphere. An array of four doubles, corresponding to x, y, z and radius of the minimum bounding sphere of a node. For a global scene, i.e. ellipsoidal coordinate systems, the values of the array correspond to longitude in decimal degrees, latitude of in decimal degrees, elevation in meters and radius in meters. For all other CRS, the values of x,y,z and r are in the same unit.
        #[serde(default)]
        pub mbs: [f64; 4],
        ///Reference to the neighbor (same level, spatial proximity) nodes of a node.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub neighbors: Vec<NodeReference>,
        ///Describes oriented bounding box.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub obb: Option<Obb>,
        ///Reference to the parent node of a node.
        #[serde(
            rename = "parentNode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub parent_node: Option<NodeReference>,
        ///Resource reference describing a shared resource document.
        #[serde(
            rename = "sharedResource",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shared_resource: Option<Resource>,
        ///Resource reference describing a texture resource.
        #[serde(
            rename = "textureData",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub texture_data: Vec<Resource>,
        ///Optional, 3D (4x4) transformation matrix expressed as a linear array of 16 values.
        #[serde(default)]
        pub transform: [f64; 16],
        ///The version (store update session ID) of this node.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<Box<str>>,
    }
    ///The node page object representing the tree as a flat array of nodes where internal nodes reference their children by their array indices.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NodePage {
        ///Array of nodes.
        pub nodes: Vec<Node>,
    }
    ///Nodes are stored contiguously in what can be considered a _flat_ array of nodes. This array can be accessed by fixed-size pages of nodes for better request efficiency. All pages contains exactly `layer.nodePages.nodesPerPage` nodes, except for the last page (that may contain less). We use an integer ID to map a node to its page as follow: ` page_id = floor( node_id / node_per_page) node_id_in_page = modulo( node_id, node_per_page) `
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NodePageDefinition {
        ///Defines the meaning of `nodes[].lodThreshold` for this layer. Possible values are: `maxScreenThreshold`: A per-node value for the maximum area of the projected bounding volume on screen in pixel. `maxScreenThresholdSQ`: A per-node value for the maximum area of the projected bounding volume on screen in pixel squared. 3D Viewers may implement **look-angle dependent** node switching by comparing this metric with the area of the 2D outline of the oriented-bounding box (OBB) on screen. ( see [_"Fast Projected Area Computation for Three-Dimensional Bounding Boxes", Dieter Schmalstieg and Robert F. Tobler_](https://pdfs.semanticscholar.org/1f59/8266e387cf367702d16acf5a4e02cc72cb99.pdf) for an efficient algorithm). If a **look-angle independent** LOD switching is desired, viewers may use the area of minimum bounding-sphere (MBS) of the node if available or the MBS of the OBB otherwise. Note: `maxScreenThresholdSQ` may be related to `maxScreenThreshold` as follow: `maxScreenThresholdSQ = PI * 0.25 * maxScreenThreshold * maxScreenThreshold`
        #[serde(rename = "lodSelectionMetricType")]
        pub lod_selection_metric_type: NodePageDefinitionLodSelectionMetricType,
        ///Number of nodes per page for this layer. **Must be a power-of-two** less than `4096`
        #[serde(rename = "nodesPerPage")]
        pub nodes_per_page: u32,
        ///Index of the root node. Default = 0.
        #[serde(rename = "rootIndex", default, skip_serializing_if = "Option::is_none")]
        pub root_index: Option<u32>,
    }
    ///A nodeReference is a pointer to another node - the parent, a child or a neighbor. A nodeReference contains a relative URL to the referenced NID, and a set of meta information which helps determines if a client loads the data and maintains store consistency.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NodeReference {
        ///Number of features in the referenced node and its descendants, down to the leaf nodes.
        #[serde(
            rename = "featureCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub feature_count: Option<u32>,
        ///Number of values per element.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub href: Option<Box<str>>,
        ///Tree Key ID of the referenced node represented as string.
        pub id: Box<str>,
        ///An array of four doubles, corresponding to x, y, z and radius of the [minimum bounding sphere](mbs.cmn.md) of a node.
        #[serde(default)]
        pub mbs: [f64; 4],
        ///Describes oriented bounding box.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub obb: Option<Obb>,
        ///Version (store update session ID) of the referenced node.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<Box<str>>,
    }
    ///An Oriented Bounding Box (OBB) is a compact bounding volume representation, tightly fitting the geometries it represents. An OBBs' invariance to translation and rotation, makes it ideal as the optimal and default bounding volume representation in I3S. When constructing an OBB for I3S use, there are two considerations an implementer needs to be make based on the Coordinate Reference System (CRS) of the layer:
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Obb {
        ///The center point of the oriented bounding box. For a global scene, such as the XY coordinate system in WGS1984, the center is specified in latitude/longitude in decimal degrees, elevation (Z) in meters.
        pub center: [f64; 3],
        ///Half size of the oriented bounding box in units of the CRS. For a global scene, such as the XY coordinate system in WGS1984, the center is specified in latitude/longitude in decimal degrees, elevation (Z) in meters.
        #[serde(rename = "halfSize")]
        pub half_size: [f64; 3],
        ///Orientation of the oriented bounding box as a 4-component quaternion. For a global scene, the quaternion is in an Earth-Centric-Earth-Fixed (ECEF) Cartesian space. ( Z+ : North, Y+ : East, X+: lon=lat=0.0). Note: A quaternion is a four-element vector that can be used to encode any rotation in a 3D coordinate system. The quaternion components are in the order x, y, z, w.
        pub quaternion: [f64; 4],
    }
    ///Feature-compatible with [glTF material](https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#materials). With the exception of emissive texture.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PbrMetallicroughness {
        ///The material's base color factor. default=`[1,1,1,1]`.
        #[serde(rename = "baseColorFactor", default)]
        pub base_color_factor: [f64; 4],
        ///The base color texture.
        #[serde(
            rename = "baseColorTexture",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub base_color_texture: Option<MaterialTexture>,
        ///The metalness of the material. default=`1.0`.
        #[serde(
            rename = "metallicFactor",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub metallic_factor: Option<f64>,
        ///The metallic-roughness texture.
        #[serde(
            rename = "metallicRoughnessTexture",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub metallic_roughness_texture: Option<MaterialTexture>,
        ///The roughness of the material. default=`1.0`.
        #[serde(
            rename = "roughnessFactor",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub roughness_factor: Option<f64>,
    }
    ///Defines the look and feel of popup windows when a user clicks or queries a feature. [See more](https://developers.arcgis.com/web-scene-specification/objects/popupInfo/) information on popup information in ArcGIS clients.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PopupInfo {
        ///A string that appears in the body of the popup window as a description. It is also possible to specify the description as HTML-formatted content.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<Box<str>>,
        ///List of Arcade expressions added to the pop-up. [See more](https://developers.arcgis.com/web-scene-specification/objects/popupExpressionInfo/) information on supported in ArcGIS clients.
        #[serde(
            rename = "expressionInfos",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub expression_infos: Vec<serde_json::Value>,
        ///Array of fieldInfo information properties. This information is provided by the service layer definition. [See more](https://developers.arcgis.com/web-scene-specification/objects/fieldInfo/) information on supported in ArcGIS clients.
        #[serde(
            rename = "fieldInfos",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub field_infos: Vec<serde_json::Value>,
        ///Array of various mediaInfo to display. Can be of type image, piechart, barchart, columnchart, or linechart. The order given is the order in which it displays. [See more](https://developers.arcgis.com/web-scene-specification/objects/mediaInfo/) information on supported in ArcGIS clients.
        #[serde(
            rename = "mediaInfos",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub media_infos: Vec<serde_json::Value>,
        ///An array of popupElement objects that represent an ordered list of popup elements. [See more](https://developers.arcgis.com/web-scene-specification/objects/popupElement/) information on supported in ArcGIS clients.
        #[serde(
            rename = "popupElements",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub popup_elements: Vec<serde_json::Value>,
        ///A string that appears at the top of the popup window as a title
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<Box<str>>,
    }
    ///Range information allows to filter features of a layer within a minimum and maximum range. Range is often used to visualize indoor spaces like picking a floor of a building or visualize rooms belonging to a specific occupation.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RangeInfo {
        ///Field name to used for the range. The statistics of the field will contain the min and max values of all features for this rangeInfo.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub field: Option<Box<str>>,
        ///A unique name that can be referenced by an application to represent the range.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<Box<str>>,
    }
    ///Resource objects are pointers to different types of resources related to a node, such as the feature data, the geometry attributes and indices, textures and shared resources.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Resource {
        ///**Deprecated.** Only applicable for geometryData resources. Represents the count of elements in faceAttributes; multiply by the sum of bytes required for each element as defined in the defaultGeometrySchema.
        #[serde(
            rename = "faceElements",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub face_elements: Vec<u32>,
        ///**Deprecated.** Only applicable for featureData resources. Provides inclusive indices of the features list in this node that indicate which features of the node are located in this bundle.
        #[serde(
            rename = "featureRange",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub feature_range: Vec<u32>,
        ///The relative URL to the referenced resource.
        pub href: Box<str>,
        ///**Deprecated.** The list of layer names that indicates which layer features in the bundle belongs to. The client can use this information to selectively download bundles.
        #[serde(
            rename = "layerContent",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub layer_content: Vec<Box<str>>,
        ///**Deprecated.** Only applicable for textureData resources. TRUE if the bundle contains multiple textures. If FALSE or not set, clients can interpret the entire bundle as a single image.
        #[serde(
            rename = "multiTextureBundle",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub multi_texture_bundle: Option<Box<str>>,
        ///**Deprecated.** Only applicable for geometryData resources. Represents the count of elements in vertexAttributes; multiply by the sum of bytes required for each element as defined in the defaultGeometrySchema.
        #[serde(
            rename = "vertexElements",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub vertex_elements: Vec<u32>,
    }
    ///The 3DSceneLayerInfo describes the properties of a layer in a store. The store object describes the exact physical storage of a layer and enables the client to detect when multiple layers are served from the same store. Every scene layer contains 3DSceneLayerInfo. If features based scene layers, such as 3D objects or point scene layers, may include the default symbology. This is as specified in the drawingInfo, which contains styling information for a feature layer. When generating 3D Objects or Integrated Mesh scene layers, the root node never has any geometry. Any node's children represent a higher LoD quality than an ancestor node. Nodes without geometry at the top of the tree are allowable since the lowest LoD of a feature/geometry is not to shown.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SceneLayerInfo {
        ///ZFactor to define conversion factor for elevation unit.
        #[serde(rename = "ZFactor", default, skip_serializing_if = "Option::is_none")]
        pub z_factor: Option<f64>,
        ///The display alias to be used for this layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<Box<str>>,
        ///Provides the schema and layout used for storing attribute content in binary format in I3S.
        #[serde(
            rename = "attributeStorageInfo",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub attribute_storage_info: Vec<AttributeStorageInfo>,
        ///Indicates if any styling information represented as drawingInfo is captured as part of the binary mesh representation. This helps provide optimal client-side access. Currently the color component of the drawingInfo is supported.
        #[serde(
            rename = "cachedDrawingInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub cached_drawing_info: Option<CachedDrawingInfo>,
        ///Capabilities supported by this layer. Possible values for each array string: `View`: View is supported. `Query`: Query is supported. `Edit`: Edit is defined. `Extract`: Extract is defined.
        pub capabilities: Vec<Capabilities>,
        ///Copyright and usage information for the data in this layer.
        #[serde(
            rename = "copyrightText",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub copyright_text: Option<Box<str>>,
        ///Description string for this layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<Box<str>>,
        ///Indicates if client application will show the popup information. Default is FALSE.
        #[serde(
            rename = "disablePopup",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub disable_popup: Option<bool>,
        ///An object containing drawing information.
        #[serde(
            rename = "drawingInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub drawing_info: Option<DrawingInfo>,
        ///An object containing elevation drawing information. If absent, any content of the scene layer is drawn at its z coordinate.
        #[serde(
            rename = "elevationInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub elevation_info: Option<ElevationInfo>,
        ///A collection of objects that describe each attribute field regarding its field name, datatype, and a user friendly name {name,type,alias}. It includes all fields that are included as part of the scene layer as derived from a source input feature layer.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub fields: Vec<Field>,
        ///3D extent. If `layer.fullExtent.spatialReference` is specified, it must match `layer.spatialReference`.
        #[serde(
            rename = "fullExtent",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub full_extent: Option<FullExtent>,
        ///Define the layouts of mesh geometry and its attributes.
        #[serde(
            rename = "geometryDefinitions",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub geometry_definitions: Vec<GeometryDefinition>,
        ///Enables consuming clients to quickly determine whether this layer is compatible (with respect to its horizontal and vertical coordinate system) with existing content.
        #[serde(
            rename = "heightModelInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub height_model_info: Option<HeightModelInfo>,
        ///The relative URL to the 3DSceneLayerResource. Only present as part of the SceneServiceInfo resource.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub href: Option<Box<str>>,
        ///Unique numeric ID of the layer.
        pub id: u32,
        ///The user-visible layer typePossible values are: `3DObject`IntegratedMesh`
        #[serde(rename = "layerType")]
        pub layer_type: LayerType,
        ///List of materials classes used in this layer.
        #[serde(
            rename = "materialDefinitions",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub material_definitions: Vec<MaterialDefinitions>,
        ///The name of this layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<Box<str>>,
        ///The paged-access index description.
        #[serde(rename = "nodePages", default, skip_serializing_if = "Option::is_none")]
        pub node_pages: Option<NodePageDefinition>,
        ///PopupInfo of the scene layer.
        #[serde(rename = "popupInfo", default, skip_serializing_if = "Option::is_none")]
        pub popup_info: Option<PopupInfo>,
        ///Range info is used to filter features of a layer withing a min and max range. The min and max range is created from the statistical information of the range field.
        #[serde(rename = "rangeInfo", default, skip_serializing_if = "Option::is_none")]
        pub range_info: Option<RangeInfo>,
        ///The time of the last update.
        #[serde(
            rename = "serviceUpdateTimeStamp",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub service_update_time_stamp: Option<ServiceUpdateTimeStamp>,
        ///The spatialReference of the layer including the vertical coordinate reference system (CRS). Well Known Text (WKT) for CRS is included to support custom CRS.
        #[serde(
            rename = "spatialReference",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub spatial_reference: Option<SpatialReference>,
        ///Contains the statistical information for a layer.
        #[serde(
            rename = "statisticsInfo",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub statistics_info: Vec<StatisticsInfo>,
        ///The store object describes the exact physical storage of a layer and enables the client to detect when multiple layers are served from the same store.
        pub store: Store,
        ///Defines the set of textures that can be referenced by meshes.
        #[serde(
            rename = "textureSetDefinitions",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub texture_set_definitions: Vec<TextureSetDefinition>,
        ///Time info represents the temporal data of a time-aware layer. The time info class provides information such as date fields that store the start and end times for each feature and the total time span for the layer.
        #[serde(rename = "timeInfo", default, skip_serializing_if = "Option::is_none")]
        pub time_info: Option<TimeInfo>,
        ///The ID of the last update session in which any resource belonging to this layer has been updated.
        pub version: Box<str>,
    }
    ///Object to provide time stamp when the I3S service or the source of the service was created or updated.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ServiceUpdateTimeStamp {
        ///Specifies the Unix epoch counting from 1 January 1970 in milliseconds. Time stamp is created when the I3S service was created or updated.
        #[serde(rename = "lastUpdate")]
        pub last_update: f64,
    }
    ///**Shared Resources are deprecated for v1.7. They must be included for backwards compatibility, but are not used.** Shared resources are models or textures that can be shared among features within the same layer. They are stored as a JSON file. Each node has a shared resource that is used by other features in the node or by features in the subtree of the current node. This approach ensures an optimal distribution of shared resources across nodes, while maintaining the node-based updating process. The SharedResource class collects Material definitions, Texture definitions, Shader definitions and geometry symbols that need to be instanced.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SharedResources {
        ///Materials describe how a Feature or a set of Features is to be rendered.
        #[serde(rename = "materialDefinitions")]
        pub material_definitions: MaterialDefinition,
        ///A Texture is a set of images, with some parameters specific to the texture/uv mapping to geometries.
        #[serde(
            rename = "textureDefinitions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub texture_definitions: Option<TextureDefinition>,
    }
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
    ///Scanning an SLPK (ZIP store) containing millions of documents is usually inefficient and slow. A hash table file may be added to the SLPK to improve first load and file scanning performances. A hash table is a data structure that implements an associative array abstract data type, a structure that can map keys to values. A hash table uses a hash function to compute an index, also called a hash code, into an array of buckets or slots, from which the desired value can be found (Wikipedia).
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SlpkHashtable {}
    ///The spatialReference object is located at the top level of the JSON hierarchy. A spatial reference can be defined using a Well-Known ID (WKID) or Well-Known Text (WKT). The default tolerance and resolution values for the associated Coordinate Reference System (CRS) are used. A spatial reference can optionally include a definition for a vertical coordinate system (VCS), which is used to interpret a geometries z values.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SpatialReference {
        ///The current WKID value of the vertical coordinate system.
        #[serde(
            rename = "latestVcsWkid",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub latest_vcs_wkid: Option<u32>,
        ///Identifies the current WKID value associated with the same spatial reference. For example a WKID of '102100' (Web Mercator) has a latestWKid of '3857'.
        #[serde(
            rename = "latestWkid",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub latest_wkid: Option<u32>,
        ///The WKID value of the vertical coordinate system.
        #[serde(rename = "vcsWkid", default, skip_serializing_if = "Option::is_none")]
        pub vcs_wkid: Option<u32>,
        ///WKID, or Well-Known ID, of the CRS. Specify either WKID or WKT of the CRS.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub wkid: Option<u32>,
        ///WKT, or Well-Known Text, of the CRS. Specify either WKT or WKID of the CRS but not both.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub wkt: Option<Box<str>>,
    }
    ///Describes the attribute statistics for the scene layer.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StatisticsInfo {
        ///The URL to the statistics information. For example ./statistics/f_1
        pub href: Box<str>,
        ///Key indicating the resource of the statistics. For example f_1 for ./statistics/f_1
        pub key: Box<str>,
        ///Name of the field of the statistical information.
        pub name: Box<str>,
    }
    ///Contains statistics about each attribute. Statistics are useful to estimate attribute distribution and range.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Stats {
        ///Contains statistics about each attribute. Statistics are useful to estimate attribute distribution and range.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stats: Option<StatsInfo>,
    }
    ///Contains statistics about each attribute. Statistics are useful to estimate attribute distribution and range. The content depends on the [field types](field.cmn.md).
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StatsInfo {
        ///Representing average or mean value. For example, sum/count.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub avg: Option<f64>,
        ///Count for the entire layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub count: Option<u32>,
        ///Represents the histogram.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub histogram: Option<Histogram>,
        ///Maximum attribute value for the entire layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max: Option<f64>,
        ///Maximum time string represented according to [time encoding](value.cmn.md). Only used for esriFieldTypeDate i3s version 1.9 or newer.
        #[serde(
            rename = "maxTimeStr",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_time_str: Option<Box<str>>,
        ///Minimum attribute value for the entire layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min: Option<f64>,
        ///Minimum time string represented according to [time encoding](value.cmn.md). Only used for esriFieldTypeDate i3s version 1.9 or newer.
        #[serde(
            rename = "minTimeStr",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub min_time_str: Option<Box<str>>,
        ///An array of most frequently used values within the point cloud scene layer.
        #[serde(
            rename = "mostFrequentValues",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub most_frequent_values: Vec<Valuecount>,
        ///Representing the standard deviation.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stddev: Option<f64>,
        ///Sum of the attribute values over the entire layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sum: Option<f64>,
        ///Represents the count of the value.
        #[serde(
            rename = "totalValuesCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub total_values_count: Option<u32>,
        ///Representing variance. For example, stats.stddev *stats.stddev.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub variance: Option<f64>,
    }
    ///The store object describes the exact physical storage of a layer and enables the client to detect when multiple layers are served from the same store. Storing multiple layers in a single store - and thus having them share resources - enables efficient serving of many layers of the same content type, but with different attribute schemas or different symbology applied.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Store {
        ///Deprecated in 1.7. MIME type for the encoding used for the Attribute Resources. For example: application/octet-stream; version=1.6.
        #[serde(
            rename = "attributeEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attribute_encoding: Option<Box<str>>,
        ///A common, global ArrayBufferView definition that can be used if the schema of vertex attributes and face attributes is consistent in an entire cache; this is a requirement for meshpyramids caches.
        #[serde(rename = "defaultGeometrySchema")]
        pub default_geometry_schema: DefaultGeometrySchema,
        ///Deprecated in 1.7. If a store uses only one material, it can be defined here entirely as a MaterialDefinition.
        #[serde(
            rename = "defaultMaterialDefinition",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_material_definition: Option<MaterialDefinition>,
        ///Deprecated in 1.7. A common, global TextureDefinition to be used for all textures in this store. The default texture definition uses a reduced profile of the full TextureDefinition, with the following attributes being mandatory: encoding, uvSet, wrap and channels.
        #[serde(
            rename = "defaultTextureDefinition",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub default_texture_definition: Vec<Texture>,
        ///The 2D spatial extent (xmin, ymin, xmax, ymax) of this store, in the horizontal indexCRS.
        #[serde(default)]
        pub extent: [f64; 4],
        ///Deprecated in 1.7. MIME type for the encoding used for the Feature Data Resources. For example: application/vnd.esri. I3S.json+gzip; version=1.6.
        #[serde(
            rename = "featureEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub feature_encoding: Option<Box<str>>,
        ///Deprecated in 1.7. MIME type for the encoding used for the Geometry Resources. For example: application/octet-stream; version=1.6.
        #[serde(
            rename = "geometryEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub geometry_encoding: Option<Box<str>>,
        ///A store ID, unique across a SceneServer. Enables the client to discover which layers are part of a common store, if any.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<Box<str>>,
        ///The horizontal CRS used for all minimum bounding spheres (mbs) in this store. The CRS is identified by an OGC URL. Needs to be identical to the spatial reference.
        #[serde(rename = "indexCRS", default, skip_serializing_if = "Option::is_none")]
        pub index_crs: Option<Box<str>>,
        ///Deprecated in 1.7. Information on the Indexing Scheme (QuadTree, R-Tree, Octree, ...) used.
        #[serde(
            rename = "indexingScheme",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub indexing_scheme: Option<Box<str>>,
        ///Deprecated in 1.7. Optional field to indicate the [LoD switching](lodSelection.cmn.md) mode. Possible values are: `node-switching`: A parent node is substituted for its children nodes when its lod threshold is exceeded. This implies that: parent and children are never shown at the same time. The bounding volumne of the parent has to enclose the features of all grandchildren. Nodes have a single parent, except the root node that have no parent. `none`: No switching model.
        #[serde(rename = "lodModel", default, skip_serializing_if = "Option::is_none")]
        pub lod_model: Option<LodModel>,
        ///Deprecated in 1.7. Optional field to indicate which LoD generation scheme is used in this store. Possible values are: `MeshPyramid`: Used for integrated mesh and 3D scene layer. `AutoThinning`: Use for point scene layer. `Clustering`: Fill in which profile types are using this lodType`Generalizing`: Fill in which profile types are using this lodType
        #[serde(rename = "lodType", default, skip_serializing_if = "Option::is_none")]
        pub lod_type: Option<LodType>,
        ///Deprecated in 1.7. MIME type for the encoding used for the Node Index Documents. Example: application/vnd.esri. I3S.json+gzip; version=1.6.
        #[serde(
            rename = "nidEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nid_encoding: Option<Box<str>>,
        ///Describes the coordinate reference frame used for storing normals. Although not required, it is recommended to re-compute the normal component of the binary geometry buffer if this property is not present. Possible values are: `east-north-up`: Normals are stored in a node local reference frame defined by the easting, northing and up directions at the MBS center. It is only valid for geographic (WGS84) vertexCRS.`earth-centered`: Normals are stored in a global earth-centered, earth-fixed ([ECEF](https://en.wikipedia.org/wiki/ECEF)) reference frame. It is only valid for geographic vertexCRS. Earth centered can be directly used for global view rendering, thus is an optimal option for meshes in non-projected geographic coordinate system. `vertex-reference-frame`: Normals are stored in the same reference frame as vertices. It is only valid for projected vertexCRS.
        #[serde(
            rename = "normalReferenceFrame",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub normal_reference_frame: Option<NormalReferenceFrame>,
        ///Indicates which profile this scene store fulfills. {point, meshpyramid, pointcloud}
        pub profile: Box<str>,
        ///Indicates the resources needed for rendering and the required order in which the client should load them. Possible values for each array string: `3dNodeIndexDocument`: JSON file describes a single index node within a store, with links to other nodes (children, sibling, and parent), links to feature data, geometry data and texture data resources, metadata such as metrics used for LoD selection, its spatial extent. [Read more](3DNodeIndexDocument.cmn.md)`SharedResource`: Shared resources are models or textures that can be shared among features within the same layer. `featureData`: The FeatureData JSON file(s) contain geographical features with a set of attributes, accessors to geometry attributes and other references to styling or materials. `Geometry`: Each geometry resource is an array of geometries. `Texture`: The texture resource for a node contains the images that are used as textures for the features stored in the node. `Attributes`: Attribute resource for node containing feature data attributes
        #[serde(
            rename = "resourcePattern",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub resource_pattern: Vec<ResourcePattern>,
        ///Relative URL to root node resource.
        #[serde(rename = "rootNode", default, skip_serializing_if = "Option::is_none")]
        pub root_node: Option<Box<str>>,
        ///Deprecated in 1.7. MIME type(s) for the encoding used for the Texture Resources.
        #[serde(
            rename = "textureEncoding",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub texture_encoding: Vec<Box<str>>,
        ///Format version of this resource. Used here again if this store hasn't been served by a 3D Scene Server.
        pub version: Box<str>,
        ///The horizontal CRS used for all 'vertex positions' in this store. The CRS is identified by an OGC URL. Needs to be identical to the spatial reference.
        #[serde(rename = "vertexCRS", default, skip_serializing_if = "Option::is_none")]
        pub vertex_crs: Option<Box<str>>,
    }
    ///The texture resource contains texture image files. Textures are stored as a binary resource within a node. I3S supports JPEG and PNG, as well as compressed texture formats S3TC, ETC2, and Basis Universal. When creating a scene layer using textures for example, a 3D Object scene layer, the appropriate texture encoding declaration needs to be provided. This is done using MIME types such as `image/jpeg` (for JPEG), `image/vnd-ms.dds` (for S3TC) and `image/ktx2` (for Basis Universal). Textures should be in RGBA format. RGBA is a three-channel RGB color model supplemented with a 4th alpha chanel. The integrated mesh and 3D object profile types support textures. The textures file is a binary resource that contains images to be used as textures for the features in the node. A single texture file contains 1 to n textures for a specific level of texture detail. It may contain a single texture or multiple individual textures. These are part of a texture atlas. Textures are expected in the following formats: |File name convention|Format| |-----|------------| |0_0.jpg|JPEG| |0.bin|PNG| |0_0_1.bin.dds|S3TC| | 0_0_2.ktx|ETC2| |1.ktx2|Basis Universal| The texture resource must include either a JPEG or PNG texture file. In I3S version 1.6, the size property will give you the width of a texture. In version 1.7, the texelCountHint can be used to determine the cost of loading a node as well as for use in texel-resolution based LoD switching. (A texel, texture element, or texture pixel is the fundamental unit of a texture map.) Compressed textures(S3TC, ETC, Basis Universal) may contain mipmaps. Mipmaps (also MIP maps) or pyramids are pre-calculated, optimized sequences of images, each of which is a progressively lower resolution representation of the same image. The height and width of each image, or level, in the mipmap is a power of two smaller than the previous level. When compressing textures with mipmaps, the texture dimensions must of size 2n and the smallest size allowed is 4x4, where n = 2. The number and volume of textures tends to be the limiting display factor, especially for web and mobile clients. The format used depends on the use case. For example, a client might choose to consume JPEG in low bandwidth conditions since JPEG encoded files are efficient to transmit and widely used. Clients constrained for memory or computing resources might choose to directly consume compressed textures for performance reasons.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Texture {
        ///True if the Map represents a texture atlas.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub atlas: Option<bool>,
        ///Indicates channels description. Possible values are: `rgb`rgba`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub channels: Option<TextureChannels>,
        ///MIMEtype[1..*] The encoding/content type that is used by all images in this map
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub encoding: Vec<Box<str>>,
        ///The name of the UV set to be used as texture coordinates.
        #[serde(rename = "uvSet", default, skip_serializing_if = "Option::is_none")]
        pub uv_set: Option<Box<str>>,
        ///Possible values for each array string: `none`repeat`mirror`
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub wrap: Vec<TextureWrap>,
    }
    ///A texture is a set of images, with some parameters specific to the texture/uv mapping to geometries. Part of [sharedResource](sharedResource.cmn.md) that is deprecated with 1.7.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TextureDefinition {
        #[serde(default, flatten)]
        pub additional_properties: std::collections::HashMap<String, TextureDefinitionInfo>,
    }
    ///A texture is a set of images, with some parameters specific to the texture/uv mapping to geometries. Part of [sharedResource](sharedResource.cmn.md) that is deprecated with 1.7.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TextureDefinitionInfo {
        ///TRUE if the Map represents a texture atlas.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub atlas: Option<bool>,
        ///Indicates channels description. Possible values are: `rgb`rgba`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub channels: Option<TextureDefinitionInfoChannels>,
        ///MIMEtype - The encoding/content type that is used by all images in this map
        pub encoding: Vec<Box<str>>,
        ///An image is a binary resource, containing a single raster that can be used to texture a feature or symbol.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub images: Vec<Image>,
        ///The name of the UV set to be used as texture coordinates.
        #[serde(rename = "uvSet", default, skip_serializing_if = "Option::is_none")]
        pub uv_set: Option<Box<str>>,
        ///UV wrapping modes, from {none, repeat, mirror}.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub wrap: Vec<TextureDefinitionInfoWrap>,
    }
    ///textureSetDefinition
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TextureSetDefinition {
        ///Set to `true` if this texture is a texture atlas. It is expected that geometries that use this texture have uv regions to specify the subtexture in the atlas.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub atlas: Option<bool>,
        ///List of formats that are available for this texture set.
        pub formats: Vec<TextureSetDefinitionFormat>,
    }
    ///textureSetDefinitionFormat
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TextureSetDefinitionFormat {
        ///The texture format. Possible values are: `jpg`: JPEG compression. No mipmaps. Please note that alpha channel may have been added after the JPEG stream. This alpha channel is alwasy 8bit and zlib compressed. Last 4 bytes of the entire stream are the 32 bit offset to the beginning of the alpha stream (little-endian).`png`: PNG format, no mipmaps`dds`: The DDS header will specify the type of compression and number of mipmaps. **WARNING:** Only DXT1 (no alpha) and DXT5 (alpha channel) are supported. `ktx-etc2`: Khronos group container for ETC2 compressed texture. Mipmap may be available. Note: KTX (Khronos Texture) is a lightweight file format for OpenGL® textures, designed around how textures are loaded in OpenGL.`ktx2`: Basis Universal Supercompressed GPU Texture.
        pub format: Format,
        ///The location ID for the resource (last segment of the URL path). Must be `"0"` for jpg/png, `"0_0_1"` for DDS, `"0_0_2"` for KTX, and `"1"` for KTX2.
        pub name: Box<str>,
    }
    ///Time info represents the temporal data of a time-aware layer. The time info provides information such as date fields storing the start and end times for each feature. The statistic of the time fields defines the time extent as a period of time with a definite start and end time. The time encoding is [ECMA ISO8601](ECMA_ISO8601.md). The date time values can be UTC time or local time with offset to UTC. Temporal data is data that represents a state in time. You can to step through periods of time to reveal patterns and trends in your data.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TimeInfo {
        ///The name of the field containing the end time information.
        #[serde(
            rename = "endTimeField",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub end_time_field: Option<Box<str>>,
        ///The name of the field that contains the start time information.
        #[serde(
            rename = "startTimeField",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub start_time_field: Option<Box<str>>,
    }
    ///Value for attributeByteCount, attributeValues and objectIds.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Value {
        ///Encoding method for the value.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encoding: Option<Box<str>>,
        ///Encoding method for the time value. DateTime attribute string formatting must comply with [ECMA-ISO 8601](ECMA_ISO8601.md). Must be: `ECMA_ISO8601`
        #[serde(
            rename = "timeEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub time_encoding: Option<TimeEncoding>,
        ///Defines the value type.
        #[serde(rename = "valueType")]
        pub value_type: Box<str>,
        ///Number of values per element.
        #[serde(
            rename = "valuesPerElement",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub values_per_element: Option<u32>,
    }
    ///A string or numeric value.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Valuecount {
        ///Count of the number of values. May exceed 32 bits.
        pub count: u64,
        ///Type of the attribute values after decompression, if applicable. Please note that `string` is not supported for point cloud scene layer attributes.
        pub value: serde_json::Value,
    }
    ///The vertexAttribute object describes valid properties for a single vertex.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VertexAttribute {
        ///The color attribute.
        pub color: GeometryAttribute,
        ///The vertex normal.
        pub normal: GeometryAttribute,
        ///The vertex position.
        pub position: GeometryAttribute,
        ///The region attribute.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<GeometryAttribute>,
        ///The first set of UV coordinates.
        pub uv0: GeometryAttribute,
    }
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
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub r#type: Option<Box<str>>,
        ///A list of Vertex Attributes, such as Position, Normals, UV coordinates, and their definitions. While there are standard keywords such as position, uv0..uv9, normal and color, this is an open, extendable list.
        #[serde(
            rename = "vertexAttributes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub vertex_attributes: Option<VertexAttribute>,
    }
}
pub mod bld {
    use super::cmn::*;
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Capabilities {
        #[serde(rename = "View")]
        View,
        #[serde(rename = "Query")]
        Query,
        #[serde(rename = "Edit")]
        Edit,
        #[serde(rename = "Extract")]
        Extract,
    }
    impl Default for Capabilities {
        fn default() -> Self {
            Self::View
        }
    }
    ///A fixed string of building information, similar to a filter. Used by client applications to define specific behavior for the modelName. The [default filter types](./defaultFilterTypes.bld.md) define the modelName for the attribute statistics. Possible values are: `category`family`familyType`bldgLevel`createdPhase`demolishedPhase`discipline`assemblyCode`omniClass`systemClassifications`systemType`systemName`systemClass`custom`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ModelName {
        #[serde(rename = "category")]
        Category,
        #[serde(rename = "family")]
        Family,
        #[serde(rename = "familyType")]
        FamilyType,
        #[serde(rename = "bldgLevel")]
        BldgLevel,
        #[serde(rename = "createdPhase")]
        CreatedPhase,
        #[serde(rename = "demolishedPhase")]
        DemolishedPhase,
        #[serde(rename = "discipline")]
        Discipline,
        #[serde(rename = "assemblyCode")]
        AssemblyCode,
        #[serde(rename = "omniClass")]
        OmniClass,
        #[serde(rename = "systemClassifications")]
        SystemClassifications,
        #[serde(rename = "systemType")]
        SystemType,
        #[serde(rename = "systemName")]
        SystemName,
        #[serde(rename = "systemClass")]
        SystemClass,
        #[serde(rename = "custom")]
        Custom,
    }
    impl Default for ModelName {
        fn default() -> Self {
            Self::Category
        }
    }
    ///Declares filter mode of type solid. Must be: `solid`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum FilterModeSolidBldType {
        #[serde(rename = "solid")]
        Solid,
    }
    impl Default for FilterModeSolidBldType {
        fn default() -> Self {
            Self::Solid
        }
    }
    ///Declares filter mode of type wire frame. Must be: `wireFrame`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum FilterModeWireFrameBldType {
        #[serde(rename = "wireFrame")]
        WireFrame,
    }
    impl Default for FilterModeWireFrameBldType {
        fn default() -> Self {
            Self::WireFrame
        }
    }
    ///Declares type or filter authoring info. Must be: `checkbox`: Client UI with checkbox representation for each filter type and filter value.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum FilterAuthoringInfoBldType {
        #[serde(rename = "checkbox")]
        Checkbox,
    }
    impl Default for FilterAuthoringInfoBldType {
        fn default() -> Self {
            Self::Checkbox
        }
    }
    ///Must be: `Building`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LayerBldLayerType {
        #[serde(rename = "Building")]
        Building,
    }
    impl Default for LayerBldLayerType {
        fn default() -> Self {
            Self::Building
        }
    }
    ///Possible values are: `group`3DObject`Point`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SublayerBldLayerType {
        #[serde(rename = "group")]
        Group,
        #[serde(rename = "3DObject")]
        ThreeDObject,
        #[serde(rename = "Point")]
        Point,
    }
    impl Default for SublayerBldLayerType {
        fn default() -> Self {
            Self::Group
        }
    }
    ///Semantics for work discipline groups which can be used to refine the user experience. Possible values are: `Mechanical`Architectural`Piping`Electrical`Structural`Infrastructure`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Discipline {
        #[serde(rename = "Mechanical")]
        Mechanical,
        #[serde(rename = "Architectural")]
        Architectural,
        #[serde(rename = "Piping")]
        Piping,
        #[serde(rename = "Electrical")]
        Electrical,
        #[serde(rename = "Structural")]
        Structural,
        #[serde(rename = "Infrastructure")]
        Infrastructure,
    }
    impl Default for Discipline {
        fn default() -> Self {
            Self::Mechanical
        }
    }
    ///Concatenated attribute statistics. If needed, the type of the attribute (string or number) may be inferred from `mostFrequentValues` and/or `min`/`max` fields.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AttributeStatisticsBld {
        ///Name of the field.
        #[serde(rename = "fieldName")]
        pub field_name: Box<str>,
        ///Label of the field name. If label is empty, the label and fieldName are identical.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub label: Option<Box<str>>,
        ///Maximum value. Numeric attributes only.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max: Option<f64>,
        ///Minimum value. Numeric attributes only.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min: Option<f64>,
        ///A fixed string of building information, similar to a filter. Used by client applications to define specific behavior for the modelName. The [default filter types](./defaultFilterTypes.bld.md) define the modelName for the attribute statistics. Possible values are: `category`family`familyType`bldgLevel`createdPhase`demolishedPhase`discipline`assemblyCode`omniClass`systemClassifications`systemType`systemName`systemClass`custom`
        #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
        pub model_name: Option<ModelName>,
        ///Most frequent value, if applicable for this attribute. Truncated to 256 entries.
        #[serde(
            rename = "mostFrequentValues",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub most_frequent_values: Vec<u32>,
        ///List of sublayers where this attribute may be found.
        #[serde(rename = "subLayerIds")]
        pub sub_layer_ids: Vec<u32>,
    }
    ///Building scene layers can be filtered by field types in the building category layer. These predefined filter types are always included in the statistical information of the building scene layer. Some filter types are common to all buildings. In addition to the default filters, other fields can be included by setting the modelName to custom. Filter types are used in the buildings [statistical information](attributestats.bld.md) as well as in the [filter authoring info](filterAuthoringInfo.bld.md). The following list contains all default filter types and modelNames when creating a building scene layer using ArcGIS.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DefaultFilterTypesBld {}
    ///The filter authoring info object contains metadata about the authoring process for creating a filter object. This allows the authoring client to save specific, overridable settings. The next time it is accessed with an authoring client, the selections are remembered. Non-authoring clients can ignore it.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FilterAuthoringInfoBld {
        ///Array of filter block authoring info.
        #[serde(rename = "filterBlocks")]
        pub filter_blocks: Vec<FilterBlockAuthoringInfoBld>,
        ///Declares type or filter authoring info. Must be: `checkbox`: Client UI with checkbox representation for each filter type and filter value.
        #[serde(rename = "type")]
        pub r#type: FilterAuthoringInfoBldType,
    }
    ///The filter object can be applied to a building scene layer. Filter allows client applications to reduce the drawn elements of a building to specific types and values.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FilterBld {
        ///Description of the filter.
        pub description: Box<str>,
        ///Authoring Info used to generate user interface for authoring clients.
        #[serde(
            rename = "filterAuthoringInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub filter_authoring_info: Option<FilterAuthoringInfoBld>,
        ///Array of filter blocks defining the filter. A filter contains at least one filter block.
        #[serde(rename = "filterBlocks")]
        pub filter_blocks: Vec<FilterBlockBld>,
        ///Global ID as unique identifier of the filter.
        pub id: Box<str>,
        ///Indicates if a filter is the default filter. Clients use the default filter to show the current state of a building. For example, if 'created' is the default filter, all elements in the 'created' phases are drawn, while elements in the 'demolished' phases are invisible. The default filter is not shown in the UI and does not have Authoring Info. (Can build specific UI for this filter)
        #[serde(
            rename = "isDefaultFilter",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub is_default_filter: Option<bool>,
        ///Defines if a filter is visible within the client application. Used to exclude filters that are overwritten from a group of filters shown in the client application.
        #[serde(rename = "isVisible", default, skip_serializing_if = "Option::is_none")]
        pub is_visible: Option<bool>,
        ///Name of the filter.
        pub name: Box<str>,
    }
    ///The filter authoring info object contains metadata about the authoring process for creating a filter block object. This allows the authoring client to save specific, overridable settings. The next time it is accessed via an authoring client, their selections are remembered. Non-authoring clients can ignore it.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FilterBlockAuthoringInfoBld {
        ///Array of defined filter types. Each filter type has an array of filter values.
        #[serde(rename = "filterTypes")]
        pub filter_types: Vec<FilterTypeBld>,
    }
    ///A filter block defines what elements will be filtered with a specific filter mode. To ensure performance on client applications, it is not recommended to declare multiple filter blocks with the same filter mode. Filter blocks are contained in a filter for a building scene layer. Each filter includes at least one filter block.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FilterBlockBld {
        ///Filter query expression for a building scene layer.
        #[serde(rename = "filterExpression")]
        pub filter_expression: Box<str>,
        ///Filter mode defines how features are drawn. For example, the filter mode of a filter can be solid or wire frame.
        #[serde(rename = "filterMode")]
        pub filter_mode: FilterModeBld,
        ///Title of the filter block.
        pub title: Box<str>,
    }
    ///Filter mode represents the way elements draw when participating in a filter block.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FilterModeBld {}
    ///Shows all elements that comply with the filter block of a filter in a building scene layer.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FilterModeSolidBld {
        ///Declares filter mode of type solid. Must be: `solid`
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub r#type: Option<FilterModeSolidBldType>,
    }
    ///Shows all elements that comply with the filter block of a filter in a building scene layer. The elements are drawn with an edge line.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FilterModeWireFrameBld {
        ///An object defining solid edges of a feature. [See more](https://developers.arcgis.com/web-scene-specification/objects/edges/) information on supported edges in ArcGIS clients.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub edges: Option<serde_json::Value>,
        ///Declares filter mode of type wire frame. Must be: `wireFrame`
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub r#type: Option<FilterModeWireFrameBldType>,
    }
    ///The file authoring information for a filter, including the filter type and its value settings.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FilterTypeBld {
        ///Represents the filter type name. Name is a unique identifier.
        #[serde(rename = "filterType")]
        pub filter_type: Box<str>,
        ///Array of filter values. Filter values are the attributes that can be stored for individual fields in a layer.
        #[serde(rename = "filterValues")]
        pub filter_values: Vec<Box<str>>,
    }
    ///#### Building scene layer structure A building scene layer represents a 3D BIM model as a single layer composed of sublayers. The 3D BIM model can be any man made structure organized in discipline layers (groups) such as Architectural, Electrical, Infrastructure, Mechanical, Piping or Structural and category layers representing content such as walls or windows. A building scene layer may contain an overview including only exterior feature in addition to the full model. The concept of a group (i.e. `layerType='group'`) has been added to organized sublayers into a nested tree structure that can be reflected in the table of content of 3D Clients. If a building scene layer does not contain an overview, the structure should not include an overview or full model, only the disciplines directly. Please note that: - Groups and sublayers may be referenced **once** (e.g. a sublayer may not be in multiple groups). - Groups and sublayers do not have any resource associated with them. - Sublayer resources are located in the sublayers of the building scene layer: layers/{bim_layer_id}/sublayers/{sub_layer_id}/.... Since a building scene layer may have an associated featureService, care must be taken to match building scene layer sublayer IDs with the service. In practice, if the building scene layer has n sublayers numbered [0,n-1] they need to match the featureService sublayers IDs. Any group layers ID in the scene layer need to be greater. ` +-- layers | +-- 10 (3dSceneLayer.json for layer10, layerType ='building' ) | | +-- statistics | | | +-- summary.json | | +-- sublayers | | | +--0 (3dSceneLayer.json for layer0, layerType='3DObject') | | | | +--nodes | | | | | +--0 | | | | | | +--3dNodeIndexDocument.json | | | | | | +--geometries (...) | | | | | | +--attributes (...) | | | | | +--1 | | | | | | +--3dNodeIndexDocument.json | | | | | | +--geometries (...) | | | | | | +--attributes (...) | | | | | +--(...) | | | | +--statistics | | | +--1 (3dSceneLayer.json for layer1, layerType='3DObject') | | | | +-- (...) | | | +--(... , layerType='3DObject') ` #### Building scene layer service: The service definition is identical to other scene layer service definitions and will list a single layer (the BIM layer) e.g: ` js { "serviceName" : "Esri Campus", "serviceVersion" : "1.8" "supportedBindings" : "REST" "layers": [ { "id" : 10, "layerType" : "Building" // ... // building scene layer JSON definitions (see example below) // ... } ] } ` **Edits** - Group/layer names **must be unique**. - Capabilities that have been removed. - `sublayers.href` and `groups.href` have been removed in favor of IDs. - Removed `fullExtent` from `group` object. - Added back `modelName`. - Added statistics.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LayerBld {
        ///Global ID, filter ID of the currently active filter for the building scene layer.
        #[serde(
            rename = "activeFilterID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub active_filter_id: Option<Box<str>>,
        ///Alias of the layer name. Can be empty if alias and name are identical.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<Box<str>>,
        ///Capabilities supported by building scene layer. Overwrites any capabilities on sublayers. Possible values for each array string: `View`: View is supported. `Query`: Query is supported. `Edit`: Edit is defined. `Extract`: Extract is defined.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub capabilities: Vec<Capabilities>,
        ///Copyright information to be displayed.
        #[serde(
            rename = "copyrightText",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub copyright_text: Option<Box<str>>,
        ///Description for the layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<Box<str>>,
        ///Array of filters defined for the building scene layer.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub filters: Vec<FilterBld>,
        ///3d extent. If `layer.fullExtent.spatialReference` is specified, it **must** match `layer.spatialReference`.
        #[serde(rename = "fullExtent")]
        pub full_extent: FullExtent,
        ///An object containing the vertical coordinate system information.
        #[serde(
            rename = "heightModelInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub height_model_info: Option<HeightModelInfo>,
        ///Identifier for the layer. Building scene layer id is not in the same namespace as sublayer id. **Important**: clients should **not** assume it will be `0`.
        pub id: u32,
        ///Must be: `Building`
        #[serde(rename = "layerType")]
        pub layer_type: LayerBldLayerType,
        ///Layer name.
        pub name: Box<str>,
        ///The spatialReference of the layer including the vertical coordinate system. WKT is included to support custom spatial references.
        #[serde(rename = "spatialReference")]
        pub spatial_reference: SpatialReference,
        ///url to statistic summary for the BIM layer. [statistics/summary.json](attributestats.bld.md)
        #[serde(
            rename = "statisticsHRef",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub statistics_h_ref: Option<Box<str>>,
        ///List of sublayers or group of sublayers.
        pub sublayers: Vec<SublayerBld>,
        ///Version of building scene layer.
        pub version: Box<str>,
    }
    ///Statistics for all building scene layer sublayers. Captures statistical information for each field in the building scene layer and the sublayers containing this fields.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StatsBld {
        ///Per-attribute statistics for all sublayers.
        pub summary: Vec<AttributeStatisticsBld>,
    }
    ///Sublayer of a building scene layer. A building scene layer is composed of an overview and the full model containing the discipline and category layers. These layer types are represented as sublayers. A sublayer may contain other layers or sublayers (i.e. `group`) to form a nested structure. The order of the layers is inverted, meaning the first layer is on the bottom and the last layer is on the top.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SublayerBld {
        ///Alias of the layer name. Can be empty if alias and name are identical.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<Box<str>>,
        ///Semantics for work discipline groups which can be used to refine the user experience. Possible values are: `Mechanical`Architectural`Piping`Electrical`Structural`Infrastructure`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discipline: Option<Discipline>,
        ///Identifier for this sublayer. **If** `layerType != 'group'`, resources will be at `/layers/{bim_layer_id}/sublayers/{this.id}/...`
        pub id: u32,
        ///Returns true if the layer has no features.
        #[serde(rename = "isEmpty", default, skip_serializing_if = "Option::is_none")]
        pub is_empty: Option<bool>,
        ///Possible values are: `group`3DObject`Point`
        #[serde(rename = "layerType")]
        pub layer_type: SublayerBldLayerType,
        ///A fixed string of sublayer information. Used by client applications to define specific behavior for the modelName. See [list of defined modelNames](subLayerModelName.md) for sublayers.
        #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
        pub model_name: Option<Box<str>>,
        ///Layer name. **Must be unique** per building scene layer.
        pub name: Box<str>,
        ///Sublayers contained in this layer.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub sublayers: Vec<SublayerBld>,
        ///Visibility of the sublayer. Default is `true`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub visibility: Option<bool>,
    }
}
pub mod psl {
    use super::cmn::*;
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Capabilities {
        #[serde(rename = "View")]
        View,
        #[serde(rename = "Query")]
        Query,
        #[serde(rename = "Edit")]
        Edit,
        #[serde(rename = "Extract")]
        Extract,
    }
    impl Default for Capabilities {
        fn default() -> Self {
            Self::View
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ResourcePattern {
        #[serde(rename = "3dNodeIndexDocument")]
        ThreeDNodeIndexDocument,
        #[serde(rename = "SharedResource")]
        SharedResource,
        #[serde(rename = "featureData")]
        FeatureData,
        #[serde(rename = "Geometry")]
        Geometry,
        #[serde(rename = "Texture")]
        Texture,
        #[serde(rename = "Attributes")]
        Attributes,
    }
    impl Default for ResourcePattern {
        fn default() -> Self {
            Self::ThreeDNodeIndexDocument
        }
    }
    ///Defines the topology type of the point. Must be: `point`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Topology {
        #[serde(rename = "point")]
        Point,
    }
    impl Default for Topology {
        fn default() -> Self {
            Self::Point
        }
    }
    ///Describes the coordinate reference frame used for storing normals. Although not required, it is recommended to re-compute the normal component of the binary geometry buffer if this property is not present. Possible values are: `east-north-up`: Normals are stored in a node local reference frame defined by the easting, northing and up directions at the MBS center. It is only valid for geographic (WGS84) vertexCRS.`earth-centered`: Normals are stored in a global earth-centered, earth-fixed ([ECEF](https://en.wikipedia.org/wiki/ECEF)) reference frame. It is only valid for geographic vertexCRS. Earth centered can be directly used for global view rendering, thus is an optimal option for meshes in non-projected geographic coordinate system. `vertex-reference-frame`: Normals are stored in the same reference frame as vertices. It is only valid for projected vertexCRS.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum NormalReferenceFrame {
        #[serde(rename = "east-north-up")]
        EastNorthUp,
        #[serde(rename = "earth-centered")]
        EarthCentered,
        #[serde(rename = "vertex-reference-frame")]
        VertexReferenceFrame,
    }
    impl Default for NormalReferenceFrame {
        fn default() -> Self {
            Self::EastNorthUp
        }
    }
    ///Optional field to indicate the [LoD switching](lodSelection.cmn.md) mode. Possible values are: `node-switching`: A parent node is substituted for its children nodes when its lod threshold is exceeded. This implies that: parent and children are never shown at the same time. The bounding volumne of the parent has to enclose the features of all grandchildren. Nodes have a single parent, except the root node that have no parent. `none`: No switching model.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LodModel {
        #[serde(rename = "node-switching")]
        NodeSwitching,
        #[serde(rename = "none")]
        None,
    }
    impl Default for LodModel {
        fn default() -> Self {
            Self::NodeSwitching
        }
    }
    ///Optional field to indicate which LoD generation scheme is used in this store. Possible values are: `MeshPyramid`: Used for integrated mesh and 3D scene layer. `AutoThinning`: Used for point scene layer. `Clustering`: Used for point cloud scene layer`Generalizing`: Used for point cloud scene layer
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LodType {
        #[serde(rename = "MeshPyramid")]
        MeshPyramid,
        #[serde(rename = "AutoThinning")]
        AutoThinning,
        #[serde(rename = "Clustering")]
        Clustering,
        #[serde(rename = "Generalizing")]
        Generalizing,
    }
    impl Default for LodType {
        fn default() -> Self {
            Self::MeshPyramid
        }
    }
    ///The user-visible layer type. Must be: `Point`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LayerType {
        #[serde(rename = "Point")]
        Point,
    }
    impl Default for LayerType {
        fn default() -> Self {
            Self::Point
        }
    }
    ///Point Geometry Description ` compressedAttributes ` **Important:** - Attribute that are present are stored continuously in the corresponding geometry buffers. - Point Geometry are always compressed
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GeometryBufferPsl {
        ///Compressed attributes. **Cannot** be combined with any other attributes.
        #[serde(
            rename = "compressedAttributes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compressed_attributes: Option<CompressedAttributes>,
    }
    ///The geometry definitions used in [Point Scene Layer]() I3S version 1.7.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GeometryDefinitionPsl {
        ///Array of geometry representation(s) for this class of points. Must be compressed.
        #[serde(rename = "geometryBuffers")]
        pub geometry_buffers: Vec<GeometryBufferPsl>,
        ///Defines the topology type of the point. Must be: `point`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub topology: Option<Topology>,
    }
    ///The resources folder is a location for additional symbology. In styles subfolder, symbols may be user defined. In this folder, root.json.gz must be defined. Root carries information such as a name(which is unique), itemtype, and more. Example of root.json ` { "items": [ { "name": "5fe9e487e2230d61de71aff13744c5e9", "title": "", "itemType": "pointSymbol", "dimensionality": "volumetric", "formats": [ "web3d", "cim" ], "cimRef": "./cim/5fe9e487e2230d61de71aff13744c5e9.json.gz", "webRef": "./web/5fe9e487e2230d61de71aff13744c5e9.json.gz", "formatInfos": [ { "type": "gltf", "href": "./gltf/5fe9e487e2230d61de71aff13744c5e9.json.gz" } ], "thumbnail": { "href": "./thumbnails/5fe9e487e2230d61de71aff13744c5e9.png" } } ], "cimVersion": "2.0.0" } ` If a symbol is defined, it is placed in a folder based on the type(gltf,jpeg,png) and given a symbolLayer json. The symbolLayer json is named based on the unique symbol name, and the resource property in the symbolLayer json is an href to an image or glb file. The supported symbol resource types are JPEG, PNG, glb.gz. The glb file type is a binary representation of 3D models saved in the gltf, then compressed with gzip. Example of the resource symbolLayer json ` { "name": "5fe9e487e2230d61de71aff13744c5e9", "type": "PointSymbol3D", "symbolLayers": [ { "type": "Object", "anchorPosition": [ 0, 0, -0.5 ], "width": 26.685164171278601, "height": 20, "depth": 64.389789603982777, "heading": -90, "anchor": "relative", "resource": { "href": "./resource/5fe9e487e2230d61de71aff13744c5e9.glb.gz" } } ] } `
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ResourcesPsl {}
    ///The 3DSceneLayerInfo object describes the properties of a layer in a store. Every scene layer contains 3DSceneLayerInfo. For features based scene layers, such as 3D objects or point scene layers, may include the default symbology, as specified in the drawingInfo, which contains stylization information for a feature layer.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SceneLayerInfoPsl {
        ///ZFactor to define conversion factor for elevation unit.
        #[serde(rename = "ZFactor", default, skip_serializing_if = "Option::is_none")]
        pub z_factor: Option<f64>,
        ///The display alias to be used for this layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<Box<str>>,
        ///Provides the schema and layout used for storing attribute content in binary format in I3S.
        #[serde(
            rename = "attributeStorageInfo",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub attribute_storage_info: Vec<AttributeStorageInfo>,
        ///Indicates if any styling information represented as drawingInfo is captured as part of the binary mesh representation. This helps provide optimal client-side access. Currently the color component of the drawingInfo is supported.
        #[serde(
            rename = "cachedDrawingInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub cached_drawing_info: Option<CachedDrawingInfo>,
        ///Capabilities supported by this layer. Possible values for each array string: `View`: View is supported. `Query`: Query is supported. `Edit`: Edit is defined. `Extract`: Extract is defined.
        pub capabilities: Vec<Capabilities>,
        ///Copyright and usage information for the data in this layer.
        #[serde(
            rename = "copyrightText",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub copyright_text: Option<Box<str>>,
        ///Description string for this layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<Box<str>>,
        ///Indicates if client application will show the popup information. Default is FALSE.
        #[serde(
            rename = "disablePopup",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub disable_popup: Option<bool>,
        ///An object containing drawing information.
        #[serde(
            rename = "drawingInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub drawing_info: Option<DrawingInfo>,
        ///An object containing elevation drawing information. If absent, any content of the scene layer is drawn at its z coordinate.
        #[serde(
            rename = "elevationInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub elevation_info: Option<ElevationInfo>,
        ///A collection of objects that describe each attribute field regarding its field name, datatype, and a user friendly name {name,type,alias}. It includes all fields that are included as part of the scene layer as derived from a source input feature layer.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub fields: Vec<Field>,
        ///3D extent. If `layer.fullExtent.spatialReference` is specified, it must match `layer.spatialReference`.
        #[serde(
            rename = "fullExtent",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub full_extent: Option<FullExtent>,
        ///Define the layouts of point geometry and its attributes.
        #[serde(
            rename = "geometryDefinitions",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub geometry_definitions: Vec<GeometryDefinitionPsl>,
        ///Enables consuming clients to quickly determine whether this layer is compatible (with respect to its horizontal and vertical coordinate system) with existing content.
        #[serde(
            rename = "heightModelInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub height_model_info: Option<HeightModelInfo>,
        ///The relative URL to the 3DSceneLayerResource. Only present as part of the SceneServiceInfo resource.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub href: Option<Box<str>>,
        ///Unique numeric ID of the layer.
        pub id: u32,
        ///The user-visible layer type. Must be: `Point`
        #[serde(rename = "layerType")]
        pub layer_type: LayerType,
        ///The name of this layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<Box<str>>,
        ///The paged-access index description. For legacy purposes, this property is called pointNodePages in [Point Scene Layers](3DSceneLayer.psl.md).
        #[serde(
            rename = "pointNodePages",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub point_node_pages: Option<NodePageDefinition>,
        ///PopupInfo of the scene layer.
        #[serde(rename = "popupInfo", default, skip_serializing_if = "Option::is_none")]
        pub popup_info: Option<PopupInfo>,
        ///Range info is used to filter features of a layer withing a min and max range. The min and max range is created from the statistical information of the range field.
        #[serde(rename = "rangeInfo", default, skip_serializing_if = "Option::is_none")]
        pub range_info: Option<RangeInfo>,
        ///The time of the last update.
        #[serde(
            rename = "serviceUpdateTimeStamp",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub service_update_time_stamp: Option<ServiceUpdateTimeStamp>,
        ///The spatialReference of the layer including the vertical coordinate reference system (CRS). Well Known Text (WKT) for CRS is included to support custom CRS.
        #[serde(
            rename = "spatialReference",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub spatial_reference: Option<SpatialReference>,
        ///Contains the statistical information for a layer.
        #[serde(
            rename = "statisticsInfo",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub statistics_info: Vec<StatisticsInfo>,
        ///The store object describes the exact physical storage of a layer and enables the client to detect when multiple layers are served from the same store.
        pub store: StorePsl,
        ///Time info represents the temporal data of a time-aware layer. The time info class provides information such as date fields that store the start and end times for each feature and the total time span for the layer.
        #[serde(rename = "timeInfo", default, skip_serializing_if = "Option::is_none")]
        pub time_info: Option<TimeInfo>,
        ///The ID of the last update session in which any resource belonging to this layer has been updated.
        pub version: Box<str>,
    }
    ///The store object describes the exact physical storage of a layer and enables the client to detect when multiple layers are served from the same store. Storing multiple layers in a single store - and thus having them share resources - enables efficient serving of many layers of the same content type, but with different attribute schemas or different symbology applied.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StorePsl {
        ///MIME type for the encoding used for the Attribute Resources. For example: application/octet-stream; version=1.6.
        #[serde(
            rename = "attributeEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attribute_encoding: Option<Box<str>>,
        ///A common, global ArrayBufferView definition that can be used if the schema of vertex attributes and face attributes is consistent in an entire cache; this is a requirement for meshpyramids caches.
        #[serde(
            rename = "defaultGeometrySchema",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_geometry_schema: Option<DefaultGeometrySchema>,
        ///If a store uses only one material, it can be defined here entirely as a MaterialDefinition.
        #[serde(
            rename = "defaultMaterialDefinition",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_material_definition: Option<MaterialDefinition>,
        ///A common, global TextureDefinition to be used for all textures in this store. The default texture definition uses a reduced profile of the full TextureDefinition, with the following attributes being mandatory: encoding, uvSet, wrap and channels.
        #[serde(
            rename = "defaultTextureDefinition",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub default_texture_definition: Vec<Texture>,
        ///The 2D spatial extent (xmin, ymin, xmax, ymax) of this store, in the horizontal indexCRS.
        #[serde(default)]
        pub extent: [f64; 4],
        ///MIME type for the encoding used for the Feature Data Resources. For example: application/vnd.esri. I3S.json+gzip; version=1.6.
        #[serde(
            rename = "featureEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub feature_encoding: Option<Box<str>>,
        ///MIME type for the encoding used for the Geometry Resources. For example: application/octet-stream; version=1.6.
        #[serde(
            rename = "geometryEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub geometry_encoding: Option<Box<str>>,
        ///A store ID, unique across a SceneServer. Enables the client to discover which layers are part of a common store, if any.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<Box<str>>,
        ///The horizontal CRS used for all minimum bounding spheres (mbs) in this store. The CRS is identified by an OGC URL. Needs to be identical to the spatial reference.
        #[serde(rename = "indexCRS", default, skip_serializing_if = "Option::is_none")]
        pub index_crs: Option<Box<str>>,
        ///Information on the Indexing Scheme (QuadTree, R-Tree, Octree, ...) used.
        #[serde(
            rename = "indexingScheme",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub indexing_scheme: Option<Box<str>>,
        ///Optional field to indicate the [LoD switching](lodSelection.cmn.md) mode. Possible values are: `node-switching`: A parent node is substituted for its children nodes when its lod threshold is exceeded. This implies that: parent and children are never shown at the same time. The bounding volumne of the parent has to enclose the features of all grandchildren. Nodes have a single parent, except the root node that have no parent. `none`: No switching model.
        #[serde(rename = "lodModel", default, skip_serializing_if = "Option::is_none")]
        pub lod_model: Option<LodModel>,
        ///Optional field to indicate which LoD generation scheme is used in this store. Possible values are: `MeshPyramid`: Used for integrated mesh and 3D scene layer. `AutoThinning`: Used for point scene layer. `Clustering`: Used for point cloud scene layer`Generalizing`: Used for point cloud scene layer
        #[serde(rename = "lodType", default, skip_serializing_if = "Option::is_none")]
        pub lod_type: Option<LodType>,
        ///MIME type for the encoding used for the Node Index Documents. Example: application/vnd.esri. I3S.json+gzip; version=1.6.
        #[serde(
            rename = "nidEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nid_encoding: Option<Box<str>>,
        ///Describes the coordinate reference frame used for storing normals. Although not required, it is recommended to re-compute the normal component of the binary geometry buffer if this property is not present. Possible values are: `east-north-up`: Normals are stored in a node local reference frame defined by the easting, northing and up directions at the MBS center. It is only valid for geographic (WGS84) vertexCRS.`earth-centered`: Normals are stored in a global earth-centered, earth-fixed ([ECEF](https://en.wikipedia.org/wiki/ECEF)) reference frame. It is only valid for geographic vertexCRS. Earth centered can be directly used for global view rendering, thus is an optimal option for meshes in non-projected geographic coordinate system. `vertex-reference-frame`: Normals are stored in the same reference frame as vertices. It is only valid for projected vertexCRS.
        #[serde(
            rename = "normalReferenceFrame",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub normal_reference_frame: Option<NormalReferenceFrame>,
        ///Indicates which profile this scene store fulfills. {point, meshpyramid, pointcloud}
        pub profile: Box<str>,
        ///Indicates the resources needed for rendering and the required order in which the client should load them. Possible values for each array string: `3dNodeIndexDocument`: JSON file describes a single index node within a store, with links to other nodes (children, sibling, and parent), links to feature data, geometry data and texture data resources, metadata such as metrics used for LoD selection, its spatial extent. [Read more](3DNodeIndexDocument.cmn.md)`SharedResource`: Shared resources are models or textures that can be shared among features within the same layer. `featureData`: The FeatureData JSON file(s) contain geographical features with a set of attributes, accessors to geometry attributes and other references to styling or materials. `Geometry`: Each geometry resource is an array of geometries. `Texture`: The texture resource for a node contains the images that are used as textures for the features stored in the node. `Attributes`: Attribute resource for node containing feature data attributes
        #[serde(
            rename = "resourcePattern",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub resource_pattern: Vec<ResourcePattern>,
        ///Relative URL to root node resource.
        #[serde(rename = "rootNode", default, skip_serializing_if = "Option::is_none")]
        pub root_node: Option<Box<str>>,
        ///MIME type(s) for the encoding used for the Texture Resources.
        #[serde(
            rename = "textureEncoding",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub texture_encoding: Vec<Box<str>>,
        ///Format version of this resource. Used here again if this store hasn't been served by a 3D Scene Server.
        pub version: Box<str>,
        ///The horizontal CRS used for all 'vertex positions' in this store. The CRS is identified by an OGC URL. Needs to be identical to the spatial reference.
        #[serde(rename = "vertexCRS", default, skip_serializing_if = "Option::is_none")]
        pub vertex_crs: Option<Box<str>>,
    }
}
pub mod pcsl {
    use super::cmn::*;
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AttributeInfoPcslOrdering {
        #[serde(rename = "attributeValues")]
        AttributeValues,
    }
    impl Default for AttributeInfoPcslOrdering {
        fn default() -> Self {
            Self::AttributeValues
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Capabilities {
        #[serde(rename = "View")]
        View,
        #[serde(rename = "Query")]
        Query,
        #[serde(rename = "Extract")]
        Extract,
    }
    impl Default for Capabilities {
        fn default() -> Self {
            Self::View
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DefaultGeometrySchemaPcslOrdering {
        #[serde(rename = "position")]
        Position,
    }
    impl Default for DefaultGeometrySchemaPcslOrdering {
        fn default() -> Self {
            Self::Position
        }
    }
    ///Defines how `node.lodThreshold` should be interpretedMust be: `density-threshold`: nodes[i].lodThreshold will represent an 'effective' 2D area for the node. This estimation works best when the point cloud scene layer represents a surface and is not volumetric. World space density is defined as Dw = node.pointCount / node.effectiveArea. Ds is Dw converted to screen space. Client would switch LOD when Ds is less/greater than a threshold defined by the client. For example, 0.1 point per pixel square. Note for point cloud scene layer creation: If each point footprint is assumed to be identical (say 0.1x0.1 unit), then the lodThreshold may be computed as number_of_points * point_footprint for a leaf node and sum( children[i].effective_area) for inner nodes.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LodSelectionMetricType {
        #[serde(rename = "density-threshold")]
        DensityThreshold,
    }
    impl Default for LodSelectionMetricType {
        fn default() -> Self {
            Self::DensityThreshold
        }
    }
    ///Defines the profile type of the scene layer as point cloud scene layer. Must be: `PointCloud`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Profile {
        #[serde(rename = "PointCloud")]
        PointCloud,
    }
    impl Default for Profile {
        fn default() -> Self {
            Self::PointCloud
        }
    }
    ///Encoding (i.e. compression) for the attribute binary buffer if different from GZIP or no-compression. Possible values are: `embedded-elevation`: No binary buffer but stats for this pseudo attribute will be available. For example, point.z from the geometry should be used. `lepcc-intensity`: LEPCC compression for scaled integral type. `lepcc-rgb`: LEPCC color compression for 3-channel RGB 8 bit.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AttributeInfoPcslEncoding {
        #[serde(rename = "embedded-elevation")]
        EmbeddedElevation,
        #[serde(rename = "lepcc-intensity")]
        LepccIntensity,
        #[serde(rename = "lepcc-rgb")]
        LepccRgb,
    }
    impl Default for AttributeInfoPcslEncoding {
        fn default() -> Self {
            Self::EmbeddedElevation
        }
    }
    ///Only 'lepcc-xyz' compression is currently supported. Must be: `lepcc-xyz`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DefaultGeometrySchemaPcslEncoding {
        #[serde(rename = "lepcc-xyz")]
        LepccXyz,
    }
    impl Default for DefaultGeometrySchemaPcslEncoding {
        fn default() -> Self {
            Self::LepccXyz
        }
    }
    ///String indicating the layer typeMust be: `PointCloud`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LayerType {
        #[serde(rename = "PointCloud")]
        PointCloud,
    }
    impl Default for LayerType {
        fn default() -> Self {
            Self::PointCloud
        }
    }
    ///The bounding volume type. Only OBB is currently supported. Must be: `obb`: Oriented bounding box
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum BoundingVolumeType {
        #[serde(rename = "obb")]
        Obb,
    }
    impl Default for BoundingVolumeType {
        fn default() -> Self {
            Self::Obb
        }
    }
    ///The type of primitive. Only points are supported for point cloud scene layer. Must be: `points`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GeometryType {
        #[serde(rename = "points")]
        Points,
    }
    impl Default for GeometryType {
        fn default() -> Self {
            Self::Points
        }
    }
    ///This property is currently **ignored* for point cloud scene layer since it only contains geometry position without vertex attributes. Must be: `PerAttributeArray`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Topology {
        #[serde(rename = "PerAttributeArray")]
        PerAttributeArray,
    }
    impl Default for Topology {
        fn default() -> Self {
            Self::PerAttributeArray
        }
    }
    ///Type of the attribute values after decompression, if applicable. Please note that `string` is not supported for point cloud scene layer attributes. Possible values are: `Int8`UInt8`Int16`UInt16`Int32`UInt32`Float32`Float64`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ValueType {
        #[serde(rename = "Int8")]
        Int8,
        #[serde(rename = "UInt8")]
        UInt8,
        #[serde(rename = "Int16")]
        Int16,
        #[serde(rename = "UInt16")]
        UInt16,
        #[serde(rename = "Int32")]
        Int32,
        #[serde(rename = "UInt32")]
        UInt32,
        #[serde(rename = "Float32")]
        Float32,
        #[serde(rename = "Float64")]
        Float64,
    }
    impl Default for ValueType {
        fn default() -> Self {
            Self::Int8
        }
    }
    ///List of attributes included for this layer.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AttributeInfoPcsl {
        ///Represents the description for value encoding, for example scalar or vector encoding.
        #[serde(
            rename = "attributeValues",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attribute_values: Option<ValuePcsl>,
        ///Encoding (i.e. compression) for the attribute binary buffer if different from GZIP or no-compression. Possible values are: `embedded-elevation`: No binary buffer but stats for this pseudo attribute will be available. For example, point.z from the geometry should be used. `lepcc-intensity`: LEPCC compression for scaled integral type. `lepcc-rgb`: LEPCC color compression for 3-channel RGB 8 bit.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encoding: Option<AttributeInfoPcslEncoding>,
        ///Represents the attribute key. Key is the same as `id' used in the resource URL to fetch the binary buffers.
        pub key: Box<str>,
        ///The attribute name. Must be unique for this layer.
        pub name: Box<str>,
        ///Mapping between attribute to point. Only 1-to-1 is currently supported. Possible values for each array string: `attributeValues`
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub ordering: Vec<AttributeInfoPcslOrdering>,
    }
    ///Example for LiDAR data: | Bit Number | Label | description | |:--:|:--:|:--| | 0| Synthetic | If set then this point was created by a technique other than LIDAR collection such as digitized from a photogrammetric stereo model or by traversing a waveform| | 1| Key-Point |If set, this point is considered to be a model key-point and thus generally should not be withheld in a thinning algorithm.| |2| Withheld |If set, this point should not be included in processing (synonymous with Deleted).| |3| Overlap | If set, this point is within the overlap region of two or more swaths or takes. Setting this bit is not mandatory (unless, of course, it is mandated by a particular delivery specification) but allows Classification of overlap points to be preserved.| |4 | Scan Channel 0 | Scanner Channel is used to indicate the channel (scanner head) of a multichannel system. Channel 0 is used for single scanner systems | |5 | Scan Channel 1 | Scanner Channel is used to indicate the channel (scanner head) of a multichannel system.| |6| Scan Direction |The Scan Direction Flag denotes the direction at which the scanner mirror was traveling at the time of the output pulse. A bit value of 1 is a positive scan direction, and a bit value of 0 is a negative scan direction (where positive scan direction is a scan moving from the left side of the in-track direction to the right side and negative the opposite). | |7| Edge of flight line | The Edge of Flight Line data bit has a value of 1 only when the point is at the end of a scan. It is the last point on a given scan line before it changes direction or the mirror facet changes. Note that this field has no meaning for 360&deg; Field of View scanners (such as Mobile LIDAR scanners) and should not be set |
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BitfieldlabelPcsl {
        ///Bit number (0 is LSB)
        #[serde(rename = "bitNumber")]
        pub bit_number: u32,
        ///Label string
        pub label: Box<str>,
    }
    ///Attribute description as field.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DefaultGeometrySchemaPcsl {
        ///Only 'lepcc-xyz' compression is currently supported. Must be: `lepcc-xyz`
        pub encoding: DefaultGeometrySchemaPcslEncoding,
        ///The type of primitive. Only points are supported for point cloud scene layer. Must be: `points`
        #[serde(rename = "geometryType")]
        pub geometry_type: GeometryType,
        ///The header in binary buffers. Currently not supported for point cloud scene layer.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub header: Vec<serde_json::Value>,
        ///Currently the geometry contains XYZ only, so vertex attribute must only list 'position'. Possible values for each array string: `position`: vertex coordinates
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub ordering: Vec<DefaultGeometrySchemaPcslOrdering>,
        ///This property is currently **ignored* for point cloud scene layer since it only contains geometry position without vertex attributes. Must be: `PerAttributeArray`
        pub topology: Topology,
        ///The vertex buffer description.
        #[serde(rename = "vertexAttributes")]
        pub vertex_attributes: VertexAttributesPcsl,
    }
    ///The drawingInfo object contains drawing information for a point cloud scene layer.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DrawingInfoPcsl {
        ///An object defining the symbology for the layer. [See more](https://developers.arcgis.com/web-scene-specification/objects/pointCloudRenderers/) information about supported renderer types in ArcGIS clients.
        pub renderer: serde_json::Value,
    }
    ///The elevationInfo defines how content in a scene layer is aligned to the ground.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ElevationInfoPcsl {
        ///The mode of the elevation. Point cloud scene layer supports absoluteHeight.
        pub mode: Box<str>,
        ///The offset the point cloud scene layer. The elevation unit is the coordinate systems units.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: Option<f64>,
    }
    ///The histogram of the point cloud scene layer. The bin size may be computed as (max-min)/bin count. Please note that stats.histo.min/max is not equivalent to stats.min/max since values smaller than stats.histo.min and greater than stats.histo.max are counted in the first and last bin respectively. The values stats.min and stats.max may be conservative estimates. The bins would be distributed as follows: `(-inf, stats.min + bin_size], (stats.min + bin_size, stats.min + 2 * bin_size], ... , (stats.min + (bin_count - 1) * bin_size], (stats.min + (bin_count - 1) * bin_size, +inf)`
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HistogramPcsl {
        ///Array of binned value counts with up to `n` values, where `n` is the number of bins and **must be less or equal to 256**.
        pub counts: Vec<u32>,
        ///Maximum value (i.e. right bound) of the last bin of the histogram.
        pub maximum: f64,
        ///Minimum value (i.e. left bound) of the first bin of the histogram.
        pub minimum: f64,
    }
    ///Describes the index (i.e. bounding volume tree) of the layer.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct IndexPcsl {
        ///The bounding volume type. Only OBB is currently supported. Must be: `obb`: Oriented bounding box
        #[serde(
            rename = "boundingVolumeType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub bounding_volume_type: Option<BoundingVolumeType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub href: Option<Box<str>>,
        ///Defines how `node.lodThreshold` should be interpretedMust be: `density-threshold`: nodes[i].lodThreshold will represent an 'effective' 2D area for the node. This estimation works best when the point cloud scene layer represents a surface and is not volumetric. World space density is defined as Dw = node.pointCount / node.effectiveArea. Ds is Dw converted to screen space. Client would switch LOD when Ds is less/greater than a threshold defined by the client. For example, 0.1 point per pixel square. Note for point cloud scene layer creation: If each point footprint is assumed to be identical (say 0.1x0.1 unit), then the lodThreshold may be computed as number_of_points * point_footprint for a leaf node and sum( children[i].effective_area) for inner nodes.
        #[serde(
            rename = "lodSelectionMetricType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub lod_selection_metric_type: Option<LodSelectionMetricType>,
        ///The version of the individual nodes format.
        #[serde(rename = "nodeVersion")]
        pub node_version: u32,
        ///The page size describes the number of nodes per paged index document. 64 is currently expected.
        #[serde(rename = "nodesPerPage")]
        pub nodes_per_page: u32,
    }
    ///Label object for the statistics labels in the point cloud profile.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LabelPcsl {
        ///Label string
        pub label: Box<str>,
        ///Value
        pub value: f64,
    }
    ///Optionally, the statistics document may contain labeling information for the attribute values.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LabelsPcsl {
        ///Array of string label/bitNumber pairs. This is useful when the attribute represent a bitfield. For example, FLAGS.
        #[serde(
            rename = "bitfieldLabels",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub bitfield_labels: Vec<BitfieldlabelPcsl>,
        ///Array of string label/value pairs. Used when attribute represents a set of values. For example, ClassCode.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub labels: Vec<LabelPcsl>,
    }
    ///Point Cloud Scene Layer Definition
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LayerPcsl {
        ///Represents the alias layer name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<Box<str>>,
        ///List of attributes included for this layer.
        #[serde(rename = "attributeStorageInfo")]
        pub attribute_storage_info: Vec<AttributeInfoPcsl>,
        ///Capabilities supported by this layer. Possible values for each array string: `View`: View is supported. `Query`: Query is supported. `Extract`: Extract is defined.
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub capabilities: Vec<Capabilities>,
        ///Copyright information to be displayed with this layer.
        #[serde(
            rename = "copyrightText",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub copyright_text: Option<Box<str>>,
        ///Description for the layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub desc: Option<Box<str>>,
        ///An object containing drawing information.
        #[serde(
            rename = "drawingInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub drawing_info: Option<DrawingInfoPcsl>,
        ///An object containing elevation information.
        #[serde(
            rename = "elevationInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub elevation_info: Option<ElevationInfoPcsl>,
        #[serde(
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub fields: Vec<Field>,
        ///An object containing the vertical coordinate system information.
        #[serde(
            rename = "heightModelInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub height_model_info: Option<HeightModelInfo>,
        ///A unique identifying number for the layer. For point cloud scene layer, only a single layer is supported, therefore, id is always 0.
        pub id: u32,
        ///String indicating the layer typeMust be: `PointCloud`
        #[serde(rename = "layerType")]
        pub layer_type: LayerType,
        ///Represents the layer name.
        pub name: Box<str>,
        ///Object to provide time stamp when the I3S service or the source of the service was created or updated.
        #[serde(
            rename = "serviceUpdateTimeStamp",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub service_update_time_stamp: Option<ServiceUpdateTimeStamp>,
        ///An object containing the WKID or WKT identifying the spatial reference of the layer's geometry.
        #[serde(rename = "spatialReference")]
        pub spatial_reference: SpatialReference,
        ///The storage for the layer.
        pub store: StorePcsl,
    }
    ///Nodes represent the spatial index of the data as a bounding volume hierarchy. To reduce the number of node-index requests required to traverse this index tree, they are organized in *pages* of [layer.index.nodesPerPage](index.pcsl.md) nodes. Children must be **contiguous**, in index range, so they may be located using `firstChild` and `childrenCount` fields. **Page Number Computation Example:** `page_id = floor( node_id / layer.store.index.nodesPerPage )` Let's say `node id` = 78 and `layer.store.index.nodesPerPage` = 64. ` page_id = floor (78 / 64) = floor (1.22) = 1 ` The `page_id` of this node is `1`. This is the second page since indexing starts at 0. **IMPORTANT:** Page size must be a power-of-two less than `4096`.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NodePageDefinitionPcsl {
        ///Array of nodes
        pub nodes: Vec<NodePcsl>,
    }
    ///A single bounding volume hierarchy node
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NodePcsl {
        ///Number of children for this node. Value is 0 if node is a leaf node.
        #[serde(rename = "childCount")]
        pub child_count: u32,
        ///Index of the first child of this node.
        #[serde(rename = "firstChild")]
        pub first_child: u32,
        ///This metric may be used as a threshold to split a parent node into its children. See [layer.store.index.lodSelectionMetricType](index.pcsl.md)
        #[serde(
            rename = "lodThreshold",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub lod_threshold: Option<f64>,
        ///Oriented bounding boxes (OBB) are the only supported bounding volumes.
        pub obb: Obb,
        ///Index of the first child of this node. The resourceID must be used to query node resources, like geometry buffer (XYZ) /nodes//geometry/0 and attribute buffers. One buffer can have one attribute. Available attributes are declared in the SceneLayer document. /nodes//attributes/.
        #[serde(rename = "resourceId")]
        pub resource_id: u32,
        ///Number of points for this node.
        #[serde(
            rename = "vertexCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub vertex_count: Option<u32>,
    }
    ///Scanning an SLPK (ZIP store) containing millions of documents is usually inefficient and slow. A hash table file may be added to the SLPK to improve first load and file scanning performances.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SlpkHashtablePcsl {}
    ///Contains statistics about each attribute. Statistics are useful to estimate attribute distribution and range. By convention, statistics are stored by attribute at `layers/0/statistics/{attribute_id}`
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StatisticsPcsl {
        ///Attribute name. Must match the name specified for this attribute in `layer.attributeStorageInfo`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub attribute: Option<Box<str>>,
        ///The statistics document may contain labeling information for the attribute values.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub labels: Option<LabelsPcsl>,
        ///Statistics for this attribute
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stats: Option<StatsPcsl>,
    }
    ///Contains statistics about each attribute. Statistics are useful to estimate attribute distribution and range.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StatsPcsl {
        ///Representing average or mean value. For example, sum/count.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub avg: Option<f64>,
        ///Count for the entire layer.
        pub count: u32,
        ///Represents the histogram.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub histogram: Option<HistogramPcsl>,
        ///(Conservative) maximum attribute value for the entire layer.
        pub max: f64,
        ///(Conservative) minimum attribute value for the entire layer.
        pub min: f64,
        ///An array of most frequently used values within the point cloud scene layer.
        #[serde(
            rename = "mostFrequentValues",
            default,
            skip_serializing_if = "is_empty_collection",
            deserialize_with = "deserialize_null_default"
        )]
        pub most_frequent_values: Vec<ValuecountPcsl>,
        ///Representing the standard deviation.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stddev: Option<f64>,
        ///Sum of the attribute values over the entire layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sum: Option<f64>,
        ///Representing variance. For example, stats.stddev *stats.stddev.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub variance: Option<f64>,
    }
    ///Describes storage for the layer.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StorePcsl {
        ///MIME type for the encoding used for the Attribute Resources. For example: application/octet-stream; version=1.6.
        #[serde(
            rename = "attributeEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attribute_encoding: Option<Box<str>>,
        ///Attribute description as field.
        #[serde(rename = "defaultGeometrySchema")]
        pub default_geometry_schema: DefaultGeometrySchemaPcsl,
        ///2D extent of the point cloud scene layer in the layers spatial reference units.
        pub extent: [f64; 4],
        ///MIME type for the encoding used for the Geometry Resources. For example: application/octet-stream; version=1.6.
        #[serde(
            rename = "geometryEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub geometry_encoding: Option<Box<str>>,
        ///Id for the store. Not currently used by the point cloud scene layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<Box<str>>,
        ///Describes the index (i.e. bounding volume tree) of the layer.
        pub index: IndexPcsl,
        ///Defines the profile type of the scene layer as point cloud scene layer. Must be: `PointCloud`
        pub profile: Profile,
        ///Point cloud scene layer store version.
        pub version: Box<str>,
    }
    ///A scalar or vector value.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ValuePcsl {
        ///Type of the attribute values after decompression, if applicable. Please note that `string` is not supported for point cloud scene layer attributes. Possible values are: `Int8`UInt8`Int16`UInt16`Int32`UInt32`Float32`Float64`
        #[serde(rename = "valueType")]
        pub value_type: ValueType,
        ///Number of components.
        #[serde(rename = "valuesPerElement")]
        pub values_per_element: u32,
    }
    ///A scalar or vector value.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ValuecountPcsl {
        ///Count the number of values. May exceed 32 bit.
        pub count: u32,
        ///Type of the attribute values after decompression, if applicable. Please note that `string` is not supported for point cloud scene layer attributes.
        pub value: f64,
    }
    ///The vertex buffer description.
    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VertexAttributesPcsl {
        ///Only LEPCC compressed (X,Y,Z) is supported. Decompressed data will be absolute `Float64` position.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub position: Option<ValuePcsl>,
    }
}

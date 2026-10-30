#[allow(unused_imports)]
use super::*;
///Declaration of the attributes per feature in the geometry, such as feature ID or face range.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureAttribute {
    ///Describes the face range of the feature attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    ///An array of six doubles, corresponding to xmin, ymin, zmin, xmax, ymax and zmax of the minimum bounding box of the feature, expressed in the vertexCRS, without offset. The mbb can be used with the Feature’s Transform to provide a LOD0 representation without loading the GeometryAttributes.
    #[serde(default)]
    pub mbb: [f64; 6],
    ///An array of three doubles, providing an optional, 'semantic' pivot offset that can be used to e.g. correctly drape tree symbols.
    #[serde(default)]
    pub pivot_offset: [f64; 3],
    ///An array of two or three doubles, giving the x,y(,z) (easting/northing/elevation) position of this feature's minimum bounding sphere center, in the vertexCRS.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub position: Vec<f64>,
}

#[allow(unused_imports)]
use super::*;
///Declaration of the attributes per feature in the geometry, such as feature ID or face range.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features {
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub feature_data: Vec<FeatureData>,
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub geometry_data: Vec<Geometry>,
}

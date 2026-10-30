#[allow(unused_imports)]
use super::*;
///A string or numeric value.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Valuecount {
    ///Count of the number of values. May exceed 32 bits.
    pub count: u64,
    ///Type of the attribute values after decompression, if applicable. Please note that `string` is not supported for point cloud scene layer attributes.
    pub value: serde_json::Value,
}

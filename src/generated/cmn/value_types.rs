#[allow(unused_imports)]
use super::*;
///Value for attributeByteCount, attributeValues and objectIds.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    ///Encoding method for the value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Box<str>>,
    ///Encoding method for the time value. DateTime attribute string formatting must comply with [ECMA-ISO 8601](ECMA_ISO8601.md). Must be: `ECMA_ISO8601`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_encoding: Option<TimeEncoding>,
    ///Defines the value type.
    pub value_type: Box<str>,
    ///Number of values per element.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values_per_element: Option<u32>,
}

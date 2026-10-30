#[allow(unused_imports)]
use super::*;
///Encoding method for the value. Possible values are: `count`: Should always be present and indicates the count of features in the attribute storage. `attributeValuesByteCount`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

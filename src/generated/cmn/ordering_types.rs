#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[allow(unused_imports)]
use super::*;
///Must be: `draco`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompressedAttributesEncoding {
    #[serde(rename = "draco")]
    Draco,
}
impl Default for CompressedAttributesEncoding {
    fn default() -> Self {
        Self::Draco
    }
}

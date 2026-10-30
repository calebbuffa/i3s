#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

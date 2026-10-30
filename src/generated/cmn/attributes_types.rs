#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

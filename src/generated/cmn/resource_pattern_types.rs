#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourcePattern {
    #[serde(rename = "3dNodeIndexDocument")]
    ThreeDNodeIndexDocument,
    #[serde(rename = "SharedResource")]
    SharedResource,
    #[serde(rename = "featureData", alias = "FeatureData")]
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

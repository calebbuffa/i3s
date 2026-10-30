#[allow(unused_imports)]
use super::*;
///The user-visible layer typePossible values are: `3DObject`IntegratedMesh`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayerType {
    #[serde(rename = "3DObject")]
    ThreeDObject,
    #[serde(rename = "IntegratedMesh")]
    IntegratedMesh,
}
impl Default for LayerType {
    fn default() -> Self {
        Self::ThreeDObject
    }
}

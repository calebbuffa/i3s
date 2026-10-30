#[allow(unused_imports)]
use super::*;
///Indicates the material type, chosen from the supported values. Possible values are: `standard`water`billboard`leafcard`reference`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaterialDefinitionInfoType {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "water")]
    Water,
    #[serde(rename = "billboard")]
    Billboard,
    #[serde(rename = "leafcard")]
    Leafcard,
    #[serde(rename = "reference")]
    Reference,
}
impl Default for MaterialDefinitionInfoType {
    fn default() -> Self {
        Self::Standard
    }
}

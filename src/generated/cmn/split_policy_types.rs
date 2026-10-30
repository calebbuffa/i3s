#[allow(unused_imports)]
use super::*;
///Split policy for the domain. Not used by Scene Layers. Possible values are: `esriSPTGeometryRatio`esriSPTDuplicate`esriSPTDefaultValue`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SplitPolicy {
    #[serde(rename = "esriSPTGeometryRatio")]
    EsriSptGeometryRatio,
    #[serde(rename = "esriSPTDuplicate")]
    EsriSptDuplicate,
    #[serde(rename = "esriSPTDefaultValue")]
    EsriSptDefaultValue,
}
impl Default for SplitPolicy {
    fn default() -> Self {
        Self::EsriSptGeometryRatio
    }
}

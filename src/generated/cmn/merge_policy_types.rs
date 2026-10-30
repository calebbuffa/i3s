#[allow(unused_imports)]
use super::*;
///Merge policy for the domain. Not used by Scene Layers. Possible values are: `esriMPTDefaultValue`esriMPTSumValues`esriMPTAreaWeighted`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MergePolicy {
    #[serde(rename = "esriMPTDefaultValue")]
    EsriMptDefaultValue,
    #[serde(rename = "esriMPTSumValues")]
    EsriMptSumValues,
    #[serde(rename = "esriMPTAreaWeighted")]
    EsriMptAreaWeighted,
}
impl Default for MergePolicy {
    fn default() -> Self {
        Self::EsriMptDefaultValue
    }
}

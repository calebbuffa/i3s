#[allow(unused_imports)]
use super::*;
///The metadata.json contains information regarding the creation and storing of i3s in SLPK to support clients with i3s service creation and processing of the data.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    ///Total number of nodes in the SLPK.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_count: Option<u32>,
}

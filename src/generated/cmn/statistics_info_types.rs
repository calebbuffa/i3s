#[allow(unused_imports)]
use super::*;
///Describes the attribute statistics for the scene layer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsInfo {
    ///The URL to the statistics information. For example ./statistics/f_1
    pub href: Box<str>,
    ///Key indicating the resource of the statistics. For example f_1 for ./statistics/f_1
    pub key: Box<str>,
    ///Name of the field of the statistical information.
    pub name: Box<str>,
}

#[allow(unused_imports)]
use super::*;
///Range information allows to filter features of a layer within a minimum and maximum range. Range is often used to visualize indoor spaces like picking a floor of a building or visualize rooms belonging to a specific occupation.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeInfo {
    ///Field name to used for the range. The statistics of the field will contain the min and max values of all features for this rangeInfo.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<Box<str>>,
    ///A unique name that can be referenced by an application to represent the range.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
}

#[allow(unused_imports)]
use super::*;
///Time info represents the temporal data of a time-aware layer. The time info provides information such as date fields storing the start and end times for each feature. The statistic of the time fields defines the time extent as a period of time with a definite start and end time. The time encoding is [ECMA ISO8601](ECMA_ISO8601.md). The date time values can be UTC time or local time with offset to UTC. Temporal data is data that represents a state in time. You can to step through periods of time to reveal patterns and trends in your data.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeInfo {
    ///The name of the field containing the end time information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time_field: Option<Box<str>>,
    ///The name of the field that contains the start time information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time_field: Option<Box<str>>,
}

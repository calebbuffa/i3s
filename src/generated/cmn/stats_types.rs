#[allow(unused_imports)]
use super::*;
///Contains statistics about each attribute. Statistics are useful to estimate attribute distribution and range.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    ///Contains statistics about each attribute. Statistics are useful to estimate attribute distribution and range.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<StatsInfo>,
}
///Contains statistics about each attribute. Statistics are useful to estimate attribute distribution and range. The content depends on the [field types](field.cmn.md).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsInfo {
    ///Representing average or mean value. For example, sum/count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg: Option<f64>,
    ///Count for the entire layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    ///Represents the histogram.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub histogram: Option<Histogram>,
    ///Maximum attribute value for the entire layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    ///Maximum time string represented according to [time encoding](value.cmn.md). Only used for esriFieldTypeDate i3s version 1.9 or newer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_time_str: Option<Box<str>>,
    ///Minimum attribute value for the entire layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    ///Minimum time string represented according to [time encoding](value.cmn.md). Only used for esriFieldTypeDate i3s version 1.9 or newer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_time_str: Option<Box<str>>,
    ///An array of most frequently used values within the point cloud scene layer.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub most_frequent_values: Vec<Valuecount>,
    ///Representing the standard deviation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stddev: Option<f64>,
    ///Sum of the attribute values over the entire layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    ///Represents the count of the value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_values_count: Option<u32>,
    ///Representing variance. For example, stats.stddev *stats.stddev.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance: Option<f64>,
}

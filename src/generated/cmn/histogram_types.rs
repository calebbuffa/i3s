#[allow(unused_imports)]
use super::*;
///The bin size may be computed as (max-min)/bin count. Please note that stats.histo.min/max is not equivalent to stats.min/max since values smaller than stats.histo.min and greater than stats.histo.max are counted in the first and last bin respectively. The values stats.min and stats.max may be conservative estimates. The bins would be distributed as follows: `(-inf, stats.min + bin_size], (stats.min + bin_size, stats.min + 2 * bin_size], ... , (stats.min + (bin_count - 1) * bin_size], (stats.min + (bin_count - 1) * bin_size, +inf)`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    ///Array of binned value counts with up to `n` values, where `n` is the number of bins and **must be less or equal to 256**.
    pub counts: Vec<u32>,
    ///Maximum value (i.e. right bound) of the last bin of the histogram.
    pub maximum: f64,
    ///Minimum value (i.e. left bound) of the first bin of the histogram.
    pub minimum: f64,
}

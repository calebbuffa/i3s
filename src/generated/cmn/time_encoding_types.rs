#[allow(unused_imports)]
use super::*;
///Encoding method for the time value. DateTime attribute string formatting must comply with [ECMA-ISO 8601](ECMA_ISO8601.md). Must be: `ECMA_ISO8601`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeEncoding {
    #[serde(rename = "ECMA_ISO8601")]
    EcmaIso8601,
}
impl Default for TimeEncoding {
    fn default() -> Self {
        Self::EcmaIso8601
    }
}

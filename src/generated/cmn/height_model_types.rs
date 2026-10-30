#[allow(unused_imports)]
use super::*;
///Represents the height model type. Possible values are: `gravity_related_height`ellipsoidal`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeightModel {
    #[serde(rename = "gravity_related_height")]
    GravityRelatedHeight,
    #[serde(rename = "ellipsoidal")]
    Ellipsoidal,
}
impl Default for HeightModel {
    fn default() -> Self {
        Self::GravityRelatedHeight
    }
}
///Represents the unit of the height. Possible values are: `meter`us-foot`foot`clarke-foot`clarke-yard`clarke-link`sears-yard`sears-foot`sears-chain`benoit-1895-b-chain`indian-yard`indian-1937-yard`gold-coast-foot`sears-1922-truncated-chain`us-inch`us-mile`us-yard`millimeter`decimeter`centimeter`kilometer`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeightUnit {
    #[serde(rename = "meter")]
    Meter,
    #[serde(rename = "us-foot")]
    UsFoot,
    #[serde(rename = "foot")]
    Foot,
    #[serde(rename = "clarke-foot")]
    ClarkeFoot,
    #[serde(rename = "clarke-yard")]
    ClarkeYard,
    #[serde(rename = "clarke-link")]
    ClarkeLink,
    #[serde(rename = "sears-yard")]
    SearsYard,
    #[serde(rename = "sears-foot")]
    SearsFoot,
    #[serde(rename = "sears-chain")]
    SearsChain,
    #[serde(rename = "benoit-1895-b-chain")]
    Benoit1895BChain,
    #[serde(rename = "indian-yard")]
    IndianYard,
    #[serde(rename = "indian-1937-yard")]
    Indian1937Yard,
    #[serde(rename = "gold-coast-foot")]
    GoldCoastFoot,
    #[serde(rename = "sears-1922-truncated-chain")]
    Sears1922TruncatedChain,
    #[serde(rename = "us-inch")]
    UsInch,
    #[serde(rename = "us-mile")]
    UsMile,
    #[serde(rename = "us-yard")]
    UsYard,
    #[serde(rename = "millimeter")]
    Millimeter,
    #[serde(rename = "decimeter")]
    Decimeter,
    #[serde(rename = "centimeter")]
    Centimeter,
    #[serde(rename = "kilometer")]
    Kilometer,
}
impl Default for HeightUnit {
    fn default() -> Self {
        Self::Meter
    }
}

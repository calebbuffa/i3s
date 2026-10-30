#[allow(unused_imports)]
use super::*;
///The I3S standard accommodates declaration of a vertical coordinate system that may either be ellipsoidal or gravity-related. This allows for a diverse range of fields and applications where the definition of elevation/height is important.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeightModelInfo {
    ///Represents the height model type. Possible values are: `gravity_related_height`ellipsoidal`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height_model: Option<HeightModel>,
    ///Represents the unit of the height. Possible values are: `meter`us-foot`foot`clarke-foot`clarke-yard`clarke-link`sears-yard`sears-foot`sears-chain`benoit-1895-b-chain`indian-yard`indian-1937-yard`gold-coast-foot`sears-1922-truncated-chain`us-inch`us-mile`us-yard`millimeter`decimeter`centimeter`kilometer`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height_unit: Option<HeightUnit>,
    ///Represents the vertical coordinate system.
    #[serde(rename = "vertCRS", default, skip_serializing_if = "Option::is_none")]
    pub vert_crs: Option<Box<str>>,
}

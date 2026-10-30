#[allow(unused_imports)]
use super::*;
///An object defining where a feature is placed within a scene. For example, on the ground or at an absolute height. [See more](https://developers.arcgis.com/web-scene-specification/objects/elevationInfo/) information on elevation in ArcGIS clients.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevationInfo {
    ///Possible values are: `relativeToGround`absoluteHeight`onTheGround`relativeToScene`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    ///Offset is always added to the result of the above logic except for onTheGround where offset is ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    ///A string value indicating the unit for the values in elevationInfo
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<Box<str>>,
}

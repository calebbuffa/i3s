#[allow(unused_imports)]
use super::*;
///The spatialReference object is located at the top level of the JSON hierarchy. A spatial reference can be defined using a Well-Known ID (WKID) or Well-Known Text (WKT). The default tolerance and resolution values for the associated Coordinate Reference System (CRS) are used. A spatial reference can optionally include a definition for a vertical coordinate system (VCS), which is used to interpret a geometries z values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialReference {
    ///The current WKID value of the vertical coordinate system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_vcs_wkid: Option<u32>,
    ///Identifies the current WKID value associated with the same spatial reference. For example a WKID of '102100' (Web Mercator) has a latestWKid of '3857'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_wkid: Option<u32>,
    ///The WKID value of the vertical coordinate system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcs_wkid: Option<u32>,
    ///WKID, or Well-Known ID, of the CRS. Specify either WKID or WKT of the CRS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wkid: Option<u32>,
    ///WKT, or Well-Known Text, of the CRS. Specify either WKT or WKID of the CRS but not both.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wkt: Option<Box<str>>,
}

#[allow(unused_imports)]
use super::*;
///The drawingInfo object contains drawing information for a scene layer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawingInfo {
    ///An object defining the symbology for the layer. [See more](https://developers.arcgis.com/web-scene-specification/objects/drawingInfo/) information about supported renderer types in ArcGIS clients.
    pub renderer: serde_json::Value,
    ///Scale symbols for the layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_symbols: Option<bool>,
}

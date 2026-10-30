#[allow(unused_imports)]
use super::*;
///Defines the look and feel of popup windows when a user clicks or queries a feature. [See more](https://developers.arcgis.com/web-scene-specification/objects/popupInfo/) information on popup information in ArcGIS clients.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PopupInfo {
    ///A string that appears in the body of the popup window as a description. It is also possible to specify the description as HTML-formatted content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    ///List of Arcade expressions added to the pop-up. [See more](https://developers.arcgis.com/web-scene-specification/objects/popupExpressionInfo/) information on supported in ArcGIS clients.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub expression_infos: Vec<serde_json::Value>,
    ///Array of fieldInfo information properties. This information is provided by the service layer definition. [See more](https://developers.arcgis.com/web-scene-specification/objects/fieldInfo/) information on supported in ArcGIS clients.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub field_infos: Vec<serde_json::Value>,
    ///Array of various mediaInfo to display. Can be of type image, piechart, barchart, columnchart, or linechart. The order given is the order in which it displays. [See more](https://developers.arcgis.com/web-scene-specification/objects/mediaInfo/) information on supported in ArcGIS clients.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub media_infos: Vec<serde_json::Value>,
    ///An array of popupElement objects that represent an ordered list of popup elements. [See more](https://developers.arcgis.com/web-scene-specification/objects/popupElement/) information on supported in ArcGIS clients.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub popup_elements: Vec<serde_json::Value>,
    ///A string that appears at the top of the popup window as a title
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
}

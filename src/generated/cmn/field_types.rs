#[allow(unused_imports)]
use super::*;
///A collection of objects describing each attribute field.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    ///Alias of the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<Box<str>>,
    ///Array of domains defined for a field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
    ///Name of the field.
    pub name: Box<str>,
    ///Type of the field. Possible values are: `esriFieldTypeDate`esriFieldTypeSingle`esriFieldTypeDouble`esriFieldTypeGUID`esriFieldTypeGlobalID`esriFieldTypeInteger`esriFieldTypeOID`esriFieldTypeSmallInteger`esriFieldTypeString`
    pub r#type: FieldType,
}

#[allow(unused_imports)]
use super::*;
///Type of the field. Possible values are: `esriFieldTypeDate`esriFieldTypeSingle`esriFieldTypeDouble`esriFieldTypeGUID`esriFieldTypeGlobalID`esriFieldTypeInteger`esriFieldTypeOID`esriFieldTypeSmallInteger`esriFieldTypeString`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FieldType {
    #[serde(rename = "esriFieldTypeDate")]
    EsriFieldTypeDate,
    #[serde(rename = "esriFieldTypeSingle")]
    EsriFieldTypeSingle,
    #[serde(rename = "esriFieldTypeDouble")]
    EsriFieldTypeDouble,
    #[serde(rename = "esriFieldTypeGUID")]
    EsriFieldTypeGuid,
    #[serde(rename = "esriFieldTypeGlobalID")]
    EsriFieldTypeGlobalId,
    #[serde(rename = "esriFieldTypeInteger")]
    EsriFieldTypeInteger,
    #[serde(rename = "esriFieldTypeOID")]
    EsriFieldTypeOid,
    #[serde(rename = "esriFieldTypeSmallInteger")]
    EsriFieldTypeSmallInteger,
    #[serde(rename = "esriFieldTypeString")]
    EsriFieldTypeString,
}
impl Default for FieldType {
    fn default() -> Self {
        Self::EsriFieldTypeDate
    }
}

#[allow(unused_imports)]
use super::*;
///The field type is the type of attribute field with which the domain can be associated. Possible values are: `esriFieldTypeDate`esriFieldTypeSingle`esriFieldTypeDouble`esriFieldTypeInteger`esriFieldTypeSmallInteger`esriFieldTypeString`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DomainFieldType {
    #[serde(rename = "esriFieldTypeDate")]
    EsriFieldTypeDate,
    #[serde(rename = "esriFieldTypeSingle")]
    EsriFieldTypeSingle,
    #[serde(rename = "esriFieldTypeDouble")]
    EsriFieldTypeDouble,
    #[serde(rename = "esriFieldTypeInteger")]
    EsriFieldTypeInteger,
    #[serde(rename = "esriFieldTypeSmallInteger")]
    EsriFieldTypeSmallInteger,
    #[serde(rename = "esriFieldTypeString")]
    EsriFieldTypeString,
}
impl Default for DomainFieldType {
    fn default() -> Self {
        Self::EsriFieldTypeDate
    }
}
///Type of domainPossible values are: `codedValue`range`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DomainType {
    #[serde(rename = "codedValue")]
    CodedValue,
    #[serde(rename = "range")]
    Range,
}
impl Default for DomainType {
    fn default() -> Self {
        Self::CodedValue
    }
}

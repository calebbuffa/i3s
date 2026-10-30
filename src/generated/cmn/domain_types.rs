#[allow(unused_imports)]
use super::*;
///Attribute domains are rules that describe the legal values of a field type, providing a method for enforcing data integrity. Attribute domains are used to constrain the values allowed in a particular attribute. Using domains helps ensure data integrity by limiting the choice of values for a particular field. Attribute domains can be shared across scene layers like 3D Object scene layers or Building Scene Layers.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    ///Range of the domain. Only string types are possible.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub coded_values: Vec<DomainCodedValue>,
    ///Description of the domain
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    ///The field type is the type of attribute field with which the domain can be associated. Possible values are: `esriFieldTypeDate`esriFieldTypeSingle`esriFieldTypeDouble`esriFieldTypeInteger`esriFieldTypeSmallInteger`esriFieldTypeString`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<DomainFieldType>,
    ///Merge policy for the domain. Not used by Scene Layers. Possible values are: `esriMPTDefaultValue`esriMPTSumValues`esriMPTAreaWeighted`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_policy: Option<MergePolicy>,
    ///Name of the domain. Must be unique per Scene Layer.
    pub name: Box<str>,
    ///Range of the domain. Only numeric types are possible.
    #[serde(default)]
    pub range: [f64; 2],
    ///Split policy for the domain. Not used by Scene Layers. Possible values are: `esriSPTGeometryRatio`esriSPTDuplicate`esriSPTDefaultValue`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_policy: Option<SplitPolicy>,
    ///Type of domainPossible values are: `codedValue`range`
    pub r#type: DomainType,
}
///Attribute domains are rules that describe the legal values of a field type, providing a method for enforcing data integrity. Attribute domains are used to constrain the values allowed in any particular attribute. Whenever a domain is associated with an attribute field, only the values within that domain are valid for the field. Using domains helps ensure data integrity by limiting the choice of values for a particular field. The domain code value contains the coded values for a domain as well as an associated description of what that value represents.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainCodedValue {
    ///Coded value (i.e. field value).
    pub code: serde_json::Value,
    ///Text representation of the domain value.
    pub name: Box<str>,
}

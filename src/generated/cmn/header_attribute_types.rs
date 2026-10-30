#[allow(unused_imports)]
use super::*;
///The header definition provides the name of each field and the value type. Headers to geometry resources must be uniform across any cache and may only contain fixed-width, single element fields.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderAttribute {
    ///The name of the property in the header.
    pub property: Box<str>,
    ///The element type of the header property. Possible values are: `UInt8`UInt16`UInt32`UInt64`Int16`Int32`Int64`Float32`Float64`
    pub r#type: HeaderAttributeType,
}
///Value for attributeByteCount, attributeValues and objectIds.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderValue {
    ///Encoding method for the value. Possible values are: `count`: Should always be present and indicates the count of features in the attribute storage. `attributeValuesByteCount`
    pub property: Property,
    ///Defines the value type. Possible values are: `Int8`UInt8`Int16`UInt16`Int32`UInt32`Float32`Float64`String`
    pub value_type: HeaderValueType,
}

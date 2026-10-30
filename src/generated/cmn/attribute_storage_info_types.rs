#[allow(unused_imports)]
use super::*;
///The attributeStorageInfo object describes the structure of the binary attribute data resource of a layer, which is the same for every node in the layer. The following examples show how different attribute types are represented as a binary buffer. # Examples of attribute resources
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeStorageInfo {
    ///For string types only. Represents the byte count of the string, including the null character.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribute_byte_counts: Option<Value>,
    ///Represents the description for value encoding. For example: scalar or vector encoding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribute_values: Option<Value>,
    ///Declares the headers of the binary attribute data.
    pub header: Vec<HeaderValue>,
    ///The unique field identifier key.
    pub key: Box<str>,
    ///The name of the field.
    pub name: Box<str>,
    ///Stores the object-id values of each feature within the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_ids: Option<Value>,
    ///Possible values for each array string: `attributeByteCounts`: Should only be present when working with string data types. `attributeValues`: Should always be present. `ObjectIds`
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub ordering: Vec<Ordering>,
}

#[allow(unused_imports)]
use super::*;
///The element type of the header property. Possible values are: `UInt8`UInt16`UInt32`UInt64`Int16`Int32`Int64`Float32`Float64`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeaderAttributeType {
    #[serde(rename = "UInt8")]
    UInt8,
    #[serde(rename = "UInt16")]
    UInt16,
    #[serde(rename = "UInt32")]
    UInt32,
    #[serde(rename = "UInt64")]
    UInt64,
    #[serde(rename = "Int16")]
    Int16,
    #[serde(rename = "Int32")]
    Int32,
    #[serde(rename = "Int64")]
    Int64,
    #[serde(rename = "Float32")]
    Float32,
    #[serde(rename = "Float64")]
    Float64,
}
impl Default for HeaderAttributeType {
    fn default() -> Self {
        Self::UInt8
    }
}
///Defines the value type. Possible values are: `Int8`UInt8`Int16`UInt16`Int32`UInt32`Float32`Float64`String`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeaderValueType {
    #[serde(rename = "Int8")]
    Int8,
    #[serde(rename = "UInt8")]
    UInt8,
    #[serde(rename = "Int16")]
    Int16,
    #[serde(rename = "UInt16")]
    UInt16,
    #[serde(rename = "Int32")]
    Int32,
    #[serde(rename = "UInt32")]
    UInt32,
    #[serde(rename = "Float32")]
    Float32,
    #[serde(rename = "Float64")]
    Float64,
    #[serde(rename = "String")]
    String,
}
impl Default for HeaderValueType {
    fn default() -> Self {
        Self::Int8
    }
}

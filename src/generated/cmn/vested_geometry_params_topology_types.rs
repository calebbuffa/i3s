#[allow(unused_imports)]
use super::*;
///Declares the typology of embedded geometry attributes or those in a geometry resources. When 'Indexed', the indices (faces) must also be declared. Possible values are: `PerAttributeArray`InterleavedArray`Indexed`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VestedGeometryParamsTopology {
    #[serde(rename = "PerAttributeArray")]
    PerAttributeArray,
    #[serde(rename = "InterleavedArray")]
    InterleavedArray,
    #[serde(rename = "Indexed")]
    Indexed,
}
impl Default for VestedGeometryParamsTopology {
    fn default() -> Self {
        Self::PerAttributeArray
    }
}

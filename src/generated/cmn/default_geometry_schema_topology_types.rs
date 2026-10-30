#[allow(unused_imports)]
use super::*;
///Declares the topology of embedded geometry attributes. When 'Indexed', the indices must also be declared in the geometry schema ('faces') and precede the vertexAttribute data. Possible values are: `PerAttributeArray`Indexed`: When Indexed, the indices must also be declared in the geometry schema (faces) and precede the vertexAttribute data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultGeometrySchemaTopology {
    #[serde(rename = "PerAttributeArray")]
    PerAttributeArray,
    #[serde(rename = "Indexed")]
    Indexed,
}
impl Default for DefaultGeometrySchemaTopology {
    fn default() -> Self {
        Self::PerAttributeArray
    }
}

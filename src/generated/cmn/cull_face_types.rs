#[allow(unused_imports)]
use super::*;
///Winding order is counterclockwise. Possible values are: `none`: Default. **Must** be none if `doubleSided=True`.`front`: Cull front faces (i.e. faces with counter-clockwise winding order).`back`: Cull back faces (i.e. faces with clockwise winding order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CullFace {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "front")]
    Front,
    #[serde(rename = "back")]
    Back,
}
impl Default for CullFace {
    fn default() -> Self {
        Self::None
    }
}

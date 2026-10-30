#[allow(unused_imports)]
use super::*;
///Rendering mode. Possible values are: `textured`solid`untextured`wireframe`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RenderMode {
    #[serde(rename = "textured")]
    Textured,
    #[serde(rename = "solid")]
    Solid,
    #[serde(rename = "untextured")]
    Untextured,
    #[serde(rename = "wireframe")]
    Wireframe,
}
impl Default for RenderMode {
    fn default() -> Self {
        Self::Textured
    }
}

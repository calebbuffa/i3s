#[allow(unused_imports)]
use super::*;
///Possible values are: `relativeToGround`absoluteHeight`onTheGround`relativeToScene`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "relativeToGround")]
    RelativeToGround,
    #[serde(rename = "absoluteHeight")]
    AbsoluteHeight,
    #[serde(rename = "onTheGround")]
    OnTheGround,
    #[serde(rename = "relativeToScene")]
    RelativeToScene,
}
impl Default for Mode {
    fn default() -> Self {
        Self::RelativeToGround
    }
}

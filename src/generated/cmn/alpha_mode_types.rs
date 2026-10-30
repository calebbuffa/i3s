#[allow(unused_imports)]
use super::*;
///Defines the meaning of the alpha-channel/alpha-mask. Possible values are: `opaque`: The rendered output is fully opaque and any alpha value is ignored. `mask`: The rendered output is either fully opaque or fully transparent depending on the alpha value and the specified alpha cutoff value. This mode is used to simulate geometry such as tree leaves or wire fences. `blend`: The rendered output is combined with the background using the normal painting operation (i.e. the Porter and Duff over operator).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlphaMode {
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "mask")]
    Mask,
    #[serde(rename = "blend")]
    Blend,
}
impl Default for AlphaMode {
    fn default() -> Self {
        Self::Opaque
    }
}

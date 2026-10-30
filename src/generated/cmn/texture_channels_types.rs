#[allow(unused_imports)]
use super::*;
///Indicates channels description. Possible values are: `rgb`rgba`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextureChannels {
    #[serde(rename = "rgb")]
    Rgb,
    #[serde(rename = "rgba")]
    Rgba,
}
impl Default for TextureChannels {
    fn default() -> Self {
        Self::Rgb
    }
}
///Indicates channels description. Possible values are: `rgb`rgba`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextureDefinitionInfoChannels {
    #[serde(rename = "rgb")]
    Rgb,
    #[serde(rename = "rgba")]
    Rgba,
}
impl Default for TextureDefinitionInfoChannels {
    fn default() -> Self {
        Self::Rgb
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextureDefinitionInfoWrap {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "repeat")]
    Repeat,
    #[serde(rename = "mirror")]
    Mirror,
}
impl Default for TextureDefinitionInfoWrap {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextureWrap {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "repeat")]
    Repeat,
    #[serde(rename = "mirror")]
    Mirror,
}
impl Default for TextureWrap {
    fn default() -> Self {
        Self::None
    }
}

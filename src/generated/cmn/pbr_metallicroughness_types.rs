#[allow(unused_imports)]
use super::*;
///Feature-compatible with [glTF material](https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#materials). With the exception of emissive texture.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PbrMetallicroughness {
    ///The material's base color factor. default=`[1,1,1,1]`.
    #[serde(default)]
    pub base_color_factor: [f64; 4],
    ///The base color texture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<MaterialTexture>,
    ///The metalness of the material. default=`1.0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metallic_factor: Option<f64>,
    ///The metallic-roughness texture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metallic_roughness_texture: Option<MaterialTexture>,
    ///The roughness of the material. default=`1.0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roughness_factor: Option<f64>,
}

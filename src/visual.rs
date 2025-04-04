//! Materials and Textures for 3D models.

use crate::options::ImageFormat;
use serde::{Deserialize, Serialize};

/// Material Texture
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialTexture {
    #[serde(rename = "textureSetDefinitionId")]
    pub texture_set_definition_id: i32,
    pub factor: Option<f64>,
}

/// Physical Based Rendering Material Definition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PBRMaterialDefinition {
    #[serde(rename = "baseColorTexture")]
    pub base_color_texture: MaterialTexture,
    #[serde(rename = "metallicFactor")]
    pub metallic_factor: f64,
    #[serde(rename = "baseColorFactor", default)]
    pub base_color_factor: Option<Vec<f64>>,
}

/// Material Definition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialDefinition {
    #[serde(rename = "doubleSided")]
    pub double_sided: bool,
    #[serde(rename = "pbrMetallicRoughness", default)]
    pub pbr_metallic_roughness: Option<PBRMaterialDefinition>,
    #[serde(rename = "metallicFactor", default)]
    pub metallic_factor: Option<f64>,
}

/// Texture Format
///
/// Parameters:
/// - `name`: The location ID for the resource (last segment of the URL path).
/// - `format`: The format of the texture.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextureFormat {
    pub name: String,
    pub format: ImageFormat,
}

impl TextureFormat {
    /// Check if the texture is compressed.
    pub fn is_compressed(&self) -> bool {
        matches!(self.format, ImageFormat::DDS | ImageFormat::KTX2)
    }

    /// Validate the texture format.
    pub fn validate(&self) -> bool {
        match self.format {
            ImageFormat::KTX2 => self.name == "1",
            ImageFormat::DDS => self.name == "0_0_1",
            ImageFormat::KtcEtc2 => self.name == "0_0_2",
            _ => self.name == "0",
        }
    }
}

/// Texture Set Definition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextureSetDefinition {
    pub formats: Vec<TextureFormat>,
    #[serde(default)]
    pub atlas: bool,
}

impl TextureSetDefinition {
    /// Check if the texture set has compressed textures.
    pub fn has_compressed(&self) -> bool {
        self.formats.len() == 2
    }
}

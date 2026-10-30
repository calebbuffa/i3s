#[allow(unused_imports)]
use super::*;
///The texture format. Possible values are: `jpg`: JPEG compression. No mipmaps. Please note that alpha channel may have been added after the JPEG stream. This alpha channel is alwasy 8bit and zlib compressed. Last 4 bytes of the entire stream are the 32 bit offset to the beginning of the alpha stream (little-endian).`png`: PNG format, no mipmaps`dds`: The DDS header will specify the type of compression and number of mipmaps. **WARNING:** Only DXT1 (no alpha) and DXT5 (alpha channel) are supported. `ktx-etc2`: Khronos group container for ETC2 compressed texture. Mipmap may be available. Note: KTX (Khronos Texture) is a lightweight file format for OpenGL® textures, designed around how textures are loaded in OpenGL.`ktx2`: Basis Universal Supercompressed GPU Texture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "jpg")]
    Jpg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "dds")]
    Dds,
    #[serde(rename = "ktx-etc2")]
    KtxEtc2,
    #[serde(rename = "ktx2")]
    Ktx2,
}
impl Default for Format {
    fn default() -> Self {
        Self::Jpg
    }
}

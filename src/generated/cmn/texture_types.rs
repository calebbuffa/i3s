#[allow(unused_imports)]
use super::*;
///The texture resource contains texture image files. Textures are stored as a binary resource within a node. I3S supports JPEG and PNG, as well as compressed texture formats S3TC, ETC2, and Basis Universal. When creating a scene layer using textures for example, a 3D Object scene layer, the appropriate texture encoding declaration needs to be provided. This is done using MIME types such as `image/jpeg` (for JPEG), `image/vnd-ms.dds` (for S3TC) and `image/ktx2` (for Basis Universal). Textures should be in RGBA format. RGBA is a three-channel RGB color model supplemented with a 4th alpha chanel. The integrated mesh and 3D object profile types support textures. The textures file is a binary resource that contains images to be used as textures for the features in the node. A single texture file contains 1 to n textures for a specific level of texture detail. It may contain a single texture or multiple individual textures. These are part of a texture atlas. Textures are expected in the following formats: |File name convention|Format| |-----|------------| |0_0.jpg|JPEG| |0.bin|PNG| |0_0_1.bin.dds|S3TC| | 0_0_2.ktx|ETC2| |1.ktx2|Basis Universal| The texture resource must include either a JPEG or PNG texture file. In I3S version 1.6, the size property will give you the width of a texture. In version 1.7, the texelCountHint can be used to determine the cost of loading a node as well as for use in texel-resolution based LoD switching. (A texel, texture element, or texture pixel is the fundamental unit of a texture map.) Compressed textures(S3TC, ETC, Basis Universal) may contain mipmaps. Mipmaps (also MIP maps) or pyramids are pre-calculated, optimized sequences of images, each of which is a progressively lower resolution representation of the same image. The height and width of each image, or level, in the mipmap is a power of two smaller than the previous level. When compressing textures with mipmaps, the texture dimensions must of size 2n and the smallest size allowed is 4x4, where n = 2. The number and volume of textures tends to be the limiting display factor, especially for web and mobile clients. The format used depends on the use case. For example, a client might choose to consume JPEG in low bandwidth conditions since JPEG encoded files are efficient to transmit and widely used. Clients constrained for memory or computing resources might choose to directly consume compressed textures for performance reasons.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Texture {
    ///True if the Map represents a texture atlas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atlas: Option<bool>,
    ///Indicates channels description. Possible values are: `rgb`rgba`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<TextureChannels>,
    ///MIMEtype\[1..*\] The encoding/content type that is used by all images in this map
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub encoding: Vec<Box<str>>,
    ///The name of the UV set to be used as texture coordinates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv_set: Option<Box<str>>,
    ///Possible values for each array string: `none`repeat`mirror`
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub wrap: Vec<TextureWrap>,
}
///A texture is a set of images, with some parameters specific to the texture/uv mapping to geometries. Part of [sharedResource](sharedResource.cmn.md) that is deprecated with 1.7.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureDefinition {
    #[serde(default, flatten)]
    pub additional_properties: std::collections::HashMap<String, TextureDefinitionInfo>,
}
///A texture is a set of images, with some parameters specific to the texture/uv mapping to geometries. Part of [sharedResource](sharedResource.cmn.md) that is deprecated with 1.7.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureDefinitionInfo {
    ///TRUE if the Map represents a texture atlas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atlas: Option<bool>,
    ///Indicates channels description. Possible values are: `rgb`rgba`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<TextureDefinitionInfoChannels>,
    ///MIMEtype - The encoding/content type that is used by all images in this map
    pub encoding: Vec<Box<str>>,
    ///An image is a binary resource, containing a single raster that can be used to texture a feature or symbol.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub images: Vec<Image>,
    ///The name of the UV set to be used as texture coordinates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv_set: Option<Box<str>>,
    ///UV wrapping modes, from {none, repeat, mirror}.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub wrap: Vec<TextureDefinitionInfoWrap>,
}
///textureSetDefinition
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureSetDefinition {
    ///Set to `true` if this texture is a texture atlas. It is expected that geometries that use this texture have uv regions to specify the subtexture in the atlas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atlas: Option<bool>,
    ///List of formats that are available for this texture set.
    pub formats: Vec<TextureSetDefinitionFormat>,
}
///textureSetDefinitionFormat
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureSetDefinitionFormat {
    ///The texture format. Possible values are: `jpg`: JPEG compression. No mipmaps. Please note that alpha channel may have been added after the JPEG stream. This alpha channel is alwasy 8bit and zlib compressed. Last 4 bytes of the entire stream are the 32 bit offset to the beginning of the alpha stream (little-endian).`png`: PNG format, no mipmaps`dds`: The DDS header will specify the type of compression and number of mipmaps. **WARNING:** Only DXT1 (no alpha) and DXT5 (alpha channel) are supported. `ktx-etc2`: Khronos group container for ETC2 compressed texture. Mipmap may be available. Note: KTX (Khronos Texture) is a lightweight file format for OpenGL® textures, designed around how textures are loaded in OpenGL.`ktx2`: Basis Universal Supercompressed GPU Texture.
    pub format: Format,
    ///The location ID for the resource (last segment of the URL path). Must be `"0"` for jpg/png, `"0_0_1"` for DDS, `"0_0_2"` for KTX, and `"1"` for KTX2.
    pub name: Box<str>,
}

#[allow(unused_imports)]
use super::*;
///An image is a binary resource, containing a single raster that can be used to texture a feature or symbol. An image represents one specific texture LoD. For details on texture organization, please refer to the section on [texture resources](texture.cmn.md).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    ///The byte offset of this image's encodings. There is one per encoding, in the same order as the encodings, in the block in which this texture image resides.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub byte_offset: Vec<u32>,
    ///The href to the image(s), one per encoding, in the same order as the encodings.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub href: Vec<Box<str>>,
    ///A unique ID for each image. Generated using the BuildID function.
    pub id: Box<str>,
    ///The length in bytes of this image's encodings. There is one per encoding, in the same order as the encodings.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub length: Vec<u32>,
    ///The maximum size of a single pixel in world units. This property is used by the client to pick the image to load and render.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pixel_in_world_units: Option<f64>,
    ///width of this image, in pixels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
}

#[allow(unused_imports)]
use super::*;
///Resource objects are pointers to different types of resources related to a node, such as the feature data, the geometry attributes and indices, textures and shared resources.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    ///**Deprecated.** Only applicable for geometryData resources. Represents the count of elements in faceAttributes; multiply by the sum of bytes required for each element as defined in the defaultGeometrySchema.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub face_elements: Vec<u32>,
    ///**Deprecated.** Only applicable for featureData resources. Provides inclusive indices of the features list in this node that indicate which features of the node are located in this bundle.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub feature_range: Vec<u32>,
    ///The relative URL to the referenced resource.
    pub href: Box<str>,
    ///**Deprecated.** The list of layer names that indicates which layer features in the bundle belongs to. The client can use this information to selectively download bundles.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub layer_content: Vec<Box<str>>,
    ///**Deprecated.** Only applicable for textureData resources. TRUE if the bundle contains multiple textures. If FALSE or not set, clients can interpret the entire bundle as a single image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_texture_bundle: Option<Box<str>>,
    ///**Deprecated.** Only applicable for geometryData resources. Represents the count of elements in vertexAttributes; multiply by the sum of bytes required for each element as defined in the defaultGeometrySchema.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub vertex_elements: Vec<u32>,
}

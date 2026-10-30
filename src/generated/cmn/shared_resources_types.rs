#[allow(unused_imports)]
use super::*;
///**Shared Resources are deprecated for v1.7. They must be included for backwards compatibility, but are not used.** Shared resources are models or textures that can be shared among features within the same layer. They are stored as a JSON file. Each node has a shared resource that is used by other features in the node or by features in the subtree of the current node. This approach ensures an optimal distribution of shared resources across nodes, while maintaining the node-based updating process. The SharedResource class collects Material definitions, Texture definitions, Shader definitions and geometry symbols that need to be instanced.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedResources {
    ///Materials describe how a Feature or a set of Features is to be rendered.
    pub material_definitions: MaterialDefinition,
    ///A Texture is a set of images, with some parameters specific to the texture/uv mapping to geometries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texture_definitions: Option<TextureDefinition>,
}

#[allow(unused_imports)]
use super::*;
///Materials describe how a feature or a set of features is to be rendered, including shading and color. Part of [sharedResource](sharedResource.cmn.md) that is deprecated with 1.7.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDefinition {
    #[serde(default, flatten)]
    pub additional_properties: std::collections::HashMap<String, MaterialDefinitionInfo>,
}
///Material information describes how a feature or a set of features is to be rendered, including shading and color. The following table provides the set of attributes and parameters for the `type`: `standard` material. Part of [sharedResource](sharedResource.cmn.md) that is deprecated with 1.7.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDefinitionInfo {
    ///The href that resolves to the shared resource bundle in which the material definition is contained.
    #[serde(rename = "$ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///A name for the material as assigned in the creating application.
    pub name: Box<str>,
    ///Parameter defined for the material.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<MaterialParams>,
    ///Indicates the material type, chosen from the supported values. Possible values are: `standard`water`billboard`leafcard`reference`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MaterialDefinitionInfoType>,
}
///The materialDefinitions object in I3S version 1.7 and higher are feature-compatible with [glTF material](https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#materials) but with the following exceptions. I3S material colors properties (baseColorFactor, emissiveFactor etc.) are assumed to be in the same color space as the textures, most commonly sRGB while in glTF they are interpreted as [linear](https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#metallic-roughness-material). glTF has separate definitions for properties like strength for [occlusionTextureInfo](https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/schema/material.occlusionTextureInfo.schema.json) and scale for [normalTextureInfo](https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/schema/material.normalTextureInfo.schema.json). Further I3S has only one [texture definition](materialTexture.cmn.md) with factor that replaces strength and scale.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDefinitions {
    ///The alpha cutoff value of the material (only applies when alphaMode=`mask`) default = `0.25`. If the alpha value is greater than or equal to the `alphaCutoff` value then it is rendered as fully opaque, otherwise, it is rendered as fully transparent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_cutoff: Option<f64>,
    ///Defines the meaning of the alpha-channel/alpha-mask. Possible values are: `opaque`: The rendered output is fully opaque and any alpha value is ignored. `mask`: The rendered output is either fully opaque or fully transparent depending on the alpha value and the specified alpha cutoff value. This mode is used to simulate geometry such as tree leaves or wire fences. `blend`: The rendered output is combined with the background using the normal painting operation (i.e. the Porter and Duff over operator).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_mode: Option<AlphaMode>,
    ///Winding order is counterclockwise. Possible values are: `none`: Default. **Must** be none if `doubleSided=True`.`front`: Cull front faces (i.e. faces with counter-clockwise winding order).`back`: Cull back faces (i.e. faces with clockwise winding order).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cull_face: Option<CullFace>,
    ///Specifies whether the material is double sided. For lighting, the opposite normals will be used when original normals are facing away from the camera. default=`false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub double_sided: Option<bool>,
    ///The emissive color of the material.
    #[serde(default)]
    pub emissive_factor: [f64; 3],
    ///The emissive texture map. A texture that receives no lighting, so the pixels are shown at full intensity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<MaterialTexture>,
    ///The normal texture map. They are a special kind of texture that allow you to add surface detail such as bumps, grooves, and scratches to a model which catch the light as if they are represented by real geometry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<MaterialTexture>,
    ///The occlusion texture map. The occlusion map is used to provide information about which areas of the model should receive high or low indirect lighting
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<MaterialTexture>,
    ///A set of parameter values that are used to define the metallic-roughness material model from Physically-Based Rendering (PBR) methodology. When not specified, all the default values of pbrMetallicRoughness apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pbr_metallic_roughness: Option<PbrMetallicroughness>,
}
///Parameters describing the material.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialParams {
    ///Ambient color of this material. Ambient color is the color of an object where it is in shadow. This color is what the object reflects when illuminated by ambient light rather than direct light.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub ambient: Vec<f64>,
    ///TRUE if features with this material should cast shadows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cast_shadows: Option<bool>,
    ///Indicates the material culling options {back, front, *none*}.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cull_face: Option<Box<str>>,
    ///Diffuse color of this material. Diffuse color is the most instinctive meaning of the color of an object. It is that essential color that the object reveals under pure white light. It is perceived as the color of the object itself rather than a reflection of the light.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub diffuse: Vec<f64>,
    ///TRUE if features with this material should receive shadows
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_shadows: Option<bool>,
    ///Indicates reflectivity of this material.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reflectivity: Option<f64>,
    ///Rendering mode. Possible values are: `textured`solid`untextured`wireframe`
    pub render_mode: RenderMode,
    ///Indicates shininess of this material.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shininess: Option<f64>,
    ///Specular color of this material. Specular color is the color of the light of a specular reflection (specular reflection is the type of reflection that is characteristic of light reflected from a shiny surface).
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub specular: Vec<f64>,
    ///Indicates transparency of this material; 0 = opaque, 1 = fully transparent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transparency: Option<f64>,
    ///Indicates whether Vertex Colors also contain a transparency channel. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_vertex_color_alpha: Option<bool>,
    ///This flag indicates that the vertex color attribute of the geometry should be used to color the geometry for rendering. If texture is present, the vertex colors are multiplied by this color. e.g. `pixel_color = [interpolated]vertex_color * texel_color`. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertex_colors: Option<bool>,
    ///This flag indicates that the geometry has uv region vertex attributes. These are used for adressing subtextures in a texture atlas. The uv coordinates are relative to this subtexture in this case. This is mostly useful for repeated textures in a texture atlas. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertex_regions: Option<bool>,
}
///The material texture definition.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialTexture {
    ///The _normal texture_: scalar multiplier applied to each normal vector of the normal texture. For _occlusion texture_,scalar multiplier controlling the amount of occlusion applied. Default=`1`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor: Option<f64>,
    ///The set index of texture's TEXCOORD attribute used for texture coordinate mapping. Default is 0. Deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tex_coord: Option<u32>,
    ///The index in [layer.textureSetDefinitions](3DSceneLayer.cmn.md).
    pub texture_set_definition_id: u32,
}

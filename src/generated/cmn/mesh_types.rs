#[allow(unused_imports)]
use super::*;
///Mesh object. Mesh geometry for a node. Clients have to use the `resource` identifiers written in each node to access the resources. While content creator may choose to match `resource` with the node id this is not required by the I3S specification and clients should not make this assumption.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mesh {
    ///The attribute set definition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribute: Option<MeshAttribute>,
    ///The geometry definition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geometry: Option<MeshGeometry>,
    ///The material definition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<MeshMaterial>,
}
///Mesh attributes for a node.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshAttribute {
    ///The resource identifier to be used to locate attribute resources of this mesh. i.e. `layers/0/nodes//attributes/...`
    pub resource: u32,
}
///Mesh geometry for a node.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshGeometry {
    ///The index in [layer.geometryDefinitions](geometryDefinition.cmn.md) array
    pub definition: u32,
    ///Number of features for this mesh. Default=`0`. (Must omit or set to `0` if mesh doesn't use `features`.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_count: Option<u32>,
    ///The resource locator to be used to query geometry resources: `layers/0/nodes/{this.resource}/geometries/{layer.geometryDefinitions[this.definition].geometryBuffers[0 or 1]}`.
    pub resource: u32,
    ///Number of vertices in the geometry buffer of this mesh for the **umcompressed mesh buffer**. Please note that `Draco` compressed meshes may have less vertices due to de-duplication (actual number of vertices is part of the Draco binary blob). Default=`0`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertex_count: Option<u32>,
}
///Mesh geometry for a node.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshMaterial {
    ///The index in [layer.materialDefinitions](3DSceneLayer.cmn.md) array.
    pub definition: u32,
    ///Resource id for the material textures. i.e: `layers/0/nodes/{material.resource}/textures/{tex_name}`. Is **required** if material declares any textures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<u32>,
    ///Estimated number of texel for the highest resolution base color texture. i.e. `texture.mip0.width*texture.mip0.height`. Useful to estimate the resource cost of this node and/or texel-resolution based LOD switching. Ignored for un-textured meshes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texel_count_hint: Option<u32>,
}

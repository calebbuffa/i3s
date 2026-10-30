#[allow(unused_imports)]
use super::*;
///Deprecated in 1.7. Optional field to indicate the [LoD switching](lodSelection.cmn.md) mode. Possible values are: `node-switching`: A parent node is substituted for its children nodes when its lod threshold is exceeded. This implies that: parent and children are never shown at the same time. The bounding volumne of the parent has to enclose the features of all grandchildren. Nodes have a single parent, except the root node that have no parent. `none`: No switching model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LodModel {
    #[serde(rename = "node-switching")]
    NodeSwitching,
    #[serde(rename = "none")]
    None,
}
impl Default for LodModel {
    fn default() -> Self {
        Self::NodeSwitching
    }
}
///Deprecated in 1.7. Optional field to indicate which LoD generation scheme is used in this store. Possible values are: `MeshPyramid`: Used for integrated mesh and 3D scene layer. `AutoThinning`: Use for point scene layer. `Clustering`: Fill in which profile types are using this lodType`Generalizing`: Fill in which profile types are using this lodType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LodType {
    #[serde(rename = "MeshPyramid")]
    MeshPyramid,
    #[serde(rename = "AutoThinning")]
    AutoThinning,
    #[serde(rename = "Clustering")]
    Clustering,
    #[serde(rename = "Generalizing")]
    Generalizing,
}
impl Default for LodType {
    fn default() -> Self {
        Self::MeshPyramid
    }
}

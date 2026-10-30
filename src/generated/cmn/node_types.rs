#[allow(unused_imports)]
use super::*;
///The node object.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    ///index of the children nodes indices.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub children: Vec<u32>,
    ///The index in the node array. May be **different than** material, geometry and attribute `resource` id. See [`mesh`](mesh.cmn.md) for more information.
    pub index: u32,
    ///When to switch LoD. See \[`nodepages[i].lodSelectionMetricType`\](nodePageDefinition.cmn.md) for more information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod_threshold: Option<f64>,
    ///The mesh for this node. **WARNING:** only **SINGLE** mesh is supported at version 1.7 (i.e. `length` **must** be 0 or 1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Mesh>,
    ///Oriented bounding box for this node.
    pub obb: Obb,
    ///The index of the parent node in the node array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_index: Option<u32>,
}
///The 3dNodeIndexDocument JSON file describes a single index node within a [store](store.cmn.md). The store object describes the exact physical storage of a layer and enables the client to detect when multiple layers are served from the same store. The file includes links to other nodes (e.g. children, sibling, and parent), links to feature data, geometry data, texture data resources, metadata (e.g. metrics used for LoD selection), and spatial extent. The node is the root object in the 3dNodeIndexDocument. There is always exactly one node object in a 3dNodeIndexDocument. Depending on the geometry and LoD model, a node document can be tuned towards being light-weight or heavy-weight. Clients decide which data to retrieve. The bounding volume information for the node, its parent, siblings, and children provide enough data for a simple visualization. For example, the centroids of a bounding volume could be rendered as point features.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeIndexDocument {
    ///Resource reference describing a featureData document.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub attribute_data: Vec<Resource>,
    ///Reference to the child nodes of a node.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub children: Vec<NodeReference>,
    ///Creation date of this node in UTC, presented as a string in the format YYYY-MM-DDThh:mm:ss.sTZD, with a fixed 'Z' time zone (see <http://www.w3.org/TR/NOTE-datetime>).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<Box<str>>,
    ///Expiration date of this node in UTC, presented as a string in the format YYYY-MM-DDThh:mm:ss.sTZD, with a fixed 'Z' time zone (see <http://www.w3.org/TR/NOTE-datetime>).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires: Option<Box<str>>,
    ///Resource reference describing a FeatureData document.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub feature_data: Vec<Resource>,
    ///**Deprecated.** A list of summary information on the features present in this node, used for pre-visualisation and LoD switching in featureTree LoD stores.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub features: Vec<Features>,
    ///Resource reference describing a geometry resource.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub geometry_data: Vec<Resource>,
    ///Tree-key ID. A unique identifier of a node within the scene layer. At 1.7 the tree-key is the integer id of the node represented as a string.
    pub id: Box<str>,
    ///Explicit level of this node within the index tree. The lowest level is 0, which is always the root node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<u32>,
    ///Metrics for LoD selection, to be evaluated by the client. *This property was previously optional which was a documentation error.
    pub lod_selection: Vec<LodSelection>,
    ///The center point of the minimum bounding sphere. An array of four doubles, corresponding to x, y, z and radius of the minimum bounding sphere of a node. For a global scene, i.e. ellipsoidal coordinate systems, the values of the array correspond to longitude in decimal degrees, latitude of in decimal degrees, elevation in meters and radius in meters. For all other CRS, the values of x,y,z and r are in the same unit.
    #[serde(default)]
    pub mbs: [f64; 4],
    ///Reference to the neighbor (same level, spatial proximity) nodes of a node.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub neighbors: Vec<NodeReference>,
    ///Describes oriented bounding box.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obb: Option<Obb>,
    ///Reference to the parent node of a node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_node: Option<NodeReference>,
    ///Resource reference describing a shared resource document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_resource: Option<Resource>,
    ///Resource reference describing a texture resource.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub texture_data: Vec<Resource>,
    ///Optional, 3D (4x4) transformation matrix expressed as a linear array of 16 values.
    #[serde(default)]
    pub transform: [f64; 16],
    ///The version (store update session ID) of this node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<str>>,
}
///The node page object representing the tree as a flat array of nodes where internal nodes reference their children by their array indices.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePage {
    ///Array of nodes.
    pub nodes: Vec<Node>,
}
///Nodes are stored contiguously in what can be considered a _flat_ array of nodes. This array can be accessed by fixed-size pages of nodes for better request efficiency. All pages contains exactly `layer.nodePages.nodesPerPage` nodes, except for the last page (that may contain less). We use an integer ID to map a node to its page as follow: ` page_id = floor( node_id / node_per_page) node_id_in_page = modulo( node_id, node_per_page) `
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePageDefinition {
    ///Defines the meaning of `nodes[].lodThreshold` for this layer. Possible values are: `maxScreenThreshold`: A per-node value for the maximum area of the projected bounding volume on screen in pixel. `maxScreenThresholdSQ`: A per-node value for the maximum area of the projected bounding volume on screen in pixel squared. 3D Viewers may implement **look-angle dependent** node switching by comparing this metric with the area of the 2D outline of the oriented-bounding box (OBB) on screen. ( see [_"Fast Projected Area Computation for Three-Dimensional Bounding Boxes", Dieter Schmalstieg and Robert F. Tobler_](https://pdfs.semanticscholar.org/1f59/8266e387cf367702d16acf5a4e02cc72cb99.pdf) for an efficient algorithm). If a **look-angle independent** LOD switching is desired, viewers may use the area of minimum bounding-sphere (MBS) of the node if available or the MBS of the OBB otherwise. Note: `maxScreenThresholdSQ` may be related to `maxScreenThreshold` as follow: `maxScreenThresholdSQ = PI * 0.25 * maxScreenThreshold * maxScreenThreshold`
    pub lod_selection_metric_type: NodePageDefinitionLodSelectionMetricType,
    ///Number of nodes per page for this layer. **Must be a power-of-two** less than `4096`
    pub nodes_per_page: u32,
    ///Index of the root node. Default = 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_index: Option<u32>,
}
///A nodeReference is a pointer to another node - the parent, a child or a neighbor. A nodeReference contains a relative URL to the referenced NID, and a set of meta information which helps determines if a client loads the data and maintains store consistency.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeReference {
    ///Number of features in the referenced node and its descendants, down to the leaf nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_count: Option<u32>,
    ///Number of values per element.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<Box<str>>,
    ///Tree Key ID of the referenced node represented as string.
    pub id: Box<str>,
    ///An array of four doubles, corresponding to x, y, z and radius of the [minimum bounding sphere](mbs.cmn.md) of a node.
    #[serde(default)]
    pub mbs: [f64; 4],
    ///Describes oriented bounding box.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obb: Option<Obb>,
    ///Version (store update session ID) of the referenced node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<str>>,
}

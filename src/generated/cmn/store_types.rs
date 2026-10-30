#[allow(unused_imports)]
use super::*;
///The store object describes the exact physical storage of a layer and enables the client to detect when multiple layers are served from the same store. Storing multiple layers in a single store - and thus having them share resources - enables efficient serving of many layers of the same content type, but with different attribute schemas or different symbology applied.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    ///Deprecated in 1.7. MIME type for the encoding used for the Attribute Resources. For example: application/octet-stream; version=1.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribute_encoding: Option<Box<str>>,
    ///A common, global ArrayBufferView definition that can be used if the schema of vertex attributes and face attributes is consistent in an entire cache; this is a requirement for meshpyramids caches.
    pub default_geometry_schema: DefaultGeometrySchema,
    ///Deprecated in 1.7. If a store uses only one material, it can be defined here entirely as a MaterialDefinition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_material_definition: Option<MaterialDefinition>,
    ///Deprecated in 1.7. A common, global TextureDefinition to be used for all textures in this store. The default texture definition uses a reduced profile of the full TextureDefinition, with the following attributes being mandatory: encoding, uvSet, wrap and channels.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub default_texture_definition: Vec<Texture>,
    ///The 2D spatial extent (xmin, ymin, xmax, ymax) of this store, in the horizontal indexCRS.
    #[serde(default)]
    pub extent: [f64; 4],
    ///Deprecated in 1.7. MIME type for the encoding used for the Feature Data Resources. For example: application/vnd.esri. I3S.json+gzip; version=1.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_encoding: Option<Box<str>>,
    ///Deprecated in 1.7. MIME type for the encoding used for the Geometry Resources. For example: application/octet-stream; version=1.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geometry_encoding: Option<Box<str>>,
    ///A store ID, unique across a SceneServer. Enables the client to discover which layers are part of a common store, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<str>>,
    ///The horizontal CRS used for all minimum bounding spheres (mbs) in this store. The CRS is identified by an OGC URL. Needs to be identical to the spatial reference.
    #[serde(rename = "indexCRS", default, skip_serializing_if = "Option::is_none")]
    pub index_crs: Option<Box<str>>,
    ///Deprecated in 1.7. Information on the Indexing Scheme (QuadTree, R-Tree, Octree, ...) used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_scheme: Option<Box<str>>,
    ///Deprecated in 1.7. Optional field to indicate the [LoD switching](lodSelection.cmn.md) mode. Possible values are: `node-switching`: A parent node is substituted for its children nodes when its lod threshold is exceeded. This implies that: parent and children are never shown at the same time. The bounding volumne of the parent has to enclose the features of all grandchildren. Nodes have a single parent, except the root node that have no parent. `none`: No switching model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod_model: Option<LodModel>,
    ///Deprecated in 1.7. Optional field to indicate which LoD generation scheme is used in this store. Possible values are: `MeshPyramid`: Used for integrated mesh and 3D scene layer. `AutoThinning`: Use for point scene layer. `Clustering`: Fill in which profile types are using this lodType`Generalizing`: Fill in which profile types are using this lodType
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod_type: Option<LodType>,
    ///Deprecated in 1.7. MIME type for the encoding used for the Node Index Documents. Example: application/vnd.esri. I3S.json+gzip; version=1.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nid_encoding: Option<Box<str>>,
    ///Describes the coordinate reference frame used for storing normals. Although not required, it is recommended to re-compute the normal component of the binary geometry buffer if this property is not present. Possible values are: `east-north-up`: Normals are stored in a node local reference frame defined by the easting, northing and up directions at the MBS center. It is only valid for geographic (WGS84) vertexCRS.`earth-centered`: Normals are stored in a global earth-centered, earth-fixed ([ECEF](https://en.wikipedia.org/wiki/ECEF)) reference frame. It is only valid for geographic vertexCRS. Earth centered can be directly used for global view rendering, thus is an optimal option for meshes in non-projected geographic coordinate system. `vertex-reference-frame`: Normals are stored in the same reference frame as vertices. It is only valid for projected vertexCRS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_reference_frame: Option<NormalReferenceFrame>,
    ///Indicates which profile this scene store fulfills. {point, meshpyramid, pointcloud}
    pub profile: Box<str>,
    ///Indicates the resources needed for rendering and the required order in which the client should load them. Possible values for each array string: `3dNodeIndexDocument`: JSON file describes a single index node within a store, with links to other nodes (children, sibling, and parent), links to feature data, geometry data and texture data resources, metadata such as metrics used for LoD selection, its spatial extent. [Read more](3DNodeIndexDocument.cmn.md)`SharedResource`: Shared resources are models or textures that can be shared among features within the same layer. `featureData`: The FeatureData JSON file(s) contain geographical features with a set of attributes, accessors to geometry attributes and other references to styling or materials. `Geometry`: Each geometry resource is an array of geometries. `Texture`: The texture resource for a node contains the images that are used as textures for the features stored in the node. `Attributes`: Attribute resource for node containing feature data attributes
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub resource_pattern: Vec<ResourcePattern>,
    ///Relative URL to root node resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_node: Option<Box<str>>,
    ///Deprecated in 1.7. MIME type(s) for the encoding used for the Texture Resources.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub texture_encoding: Vec<Box<str>>,
    ///Format version of this resource. Used here again if this store hasn't been served by a 3D Scene Server.
    pub version: Box<str>,
    ///The horizontal CRS used for all 'vertex positions' in this store. The CRS is identified by an OGC URL. Needs to be identical to the spatial reference.
    #[serde(rename = "vertexCRS", default, skip_serializing_if = "Option::is_none")]
    pub vertex_crs: Option<Box<str>>,
}

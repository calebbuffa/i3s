use super::cmn::*;
#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Capabilities {
    #[serde(rename = "View")]
    View,
    #[serde(rename = "Query")]
    Query,
    #[serde(rename = "Edit")]
    Edit,
    #[serde(rename = "Extract")]
    Extract,
}
impl Default for Capabilities {
    fn default() -> Self {
        Self::View
    }
}
///The user-visible layer type. Must be: `Point`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayerType {
    #[serde(rename = "Point")]
    Point,
}
impl Default for LayerType {
    fn default() -> Self {
        Self::Point
    }
}
///Optional field to indicate the [LoD switching](lodSelection.cmn.md) mode. Possible values are: `node-switching`: A parent node is substituted for its children nodes when its lod threshold is exceeded. This implies that: parent and children are never shown at the same time. The bounding volumne of the parent has to enclose the features of all grandchildren. Nodes have a single parent, except the root node that have no parent. `none`: No switching model.
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
///Optional field to indicate which LoD generation scheme is used in this store. Possible values are: `MeshPyramid`: Used for integrated mesh and 3D scene layer. `AutoThinning`: Used for point scene layer. `Clustering`: Used for point cloud scene layer`Generalizing`: Used for point cloud scene layer
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
///Describes the coordinate reference frame used for storing normals. Although not required, it is recommended to re-compute the normal component of the binary geometry buffer if this property is not present. Possible values are: `east-north-up`: Normals are stored in a node local reference frame defined by the easting, northing and up directions at the MBS center. It is only valid for geographic (WGS84) vertexCRS.`earth-centered`: Normals are stored in a global earth-centered, earth-fixed ([ECEF](https://en.wikipedia.org/wiki/ECEF)) reference frame. It is only valid for geographic vertexCRS. Earth centered can be directly used for global view rendering, thus is an optimal option for meshes in non-projected geographic coordinate system. `vertex-reference-frame`: Normals are stored in the same reference frame as vertices. It is only valid for projected vertexCRS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NormalReferenceFrame {
    #[serde(rename = "east-north-up")]
    EastNorthUp,
    #[serde(rename = "earth-centered")]
    EarthCentered,
    #[serde(rename = "vertex-reference-frame")]
    VertexReferenceFrame,
}
impl Default for NormalReferenceFrame {
    fn default() -> Self {
        Self::EastNorthUp
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourcePattern {
    #[serde(rename = "3dNodeIndexDocument")]
    ThreeDNodeIndexDocument,
    #[serde(rename = "SharedResource")]
    SharedResource,
    #[serde(rename = "featureData", alias = "FeatureData")]
    FeatureData,
    #[serde(rename = "Geometry")]
    Geometry,
    #[serde(rename = "Texture")]
    Texture,
    #[serde(rename = "Attributes")]
    Attributes,
}
impl Default for ResourcePattern {
    fn default() -> Self {
        Self::ThreeDNodeIndexDocument
    }
}
///Defines the topology type of the point. Must be: `point`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Topology {
    #[serde(rename = "point")]
    Point,
}
impl Default for Topology {
    fn default() -> Self {
        Self::Point
    }
}
///Point Geometry Description ` compressedAttributes ` **Important:** - Attribute that are present are stored continuously in the corresponding geometry buffers. - Point Geometry are always compressed
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryBufferPsl {
    ///Compressed attributes. **Cannot** be combined with any other attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compressed_attributes: Option<CompressedAttributes>,
}
///The geometry definitions used in [Point Scene Layer]() I3S version 1.7.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryDefinitionPsl {
    ///Array of geometry representation(s) for this class of points. Must be compressed.
    pub geometry_buffers: Vec<GeometryBufferPsl>,
    ///Defines the topology type of the point. Must be: `point`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topology: Option<Topology>,
}
///The resources folder is a location for additional symbology. In styles subfolder, symbols may be user defined. In this folder, root.json.gz must be defined. Root carries information such as a name(which is unique), itemtype, and more. Example of root.json ` { "items": [ { "name": "5fe9e487e2230d61de71aff13744c5e9", "title": "", "itemType": "pointSymbol", "dimensionality": "volumetric", "formats": [ "web3d", "cim" ], "cimRef": "./cim/5fe9e487e2230d61de71aff13744c5e9.json.gz", "webRef": "./web/5fe9e487e2230d61de71aff13744c5e9.json.gz", "formatInfos": [ { "type": "gltf", "href": "./gltf/5fe9e487e2230d61de71aff13744c5e9.json.gz" } ], "thumbnail": { "href": "./thumbnails/5fe9e487e2230d61de71aff13744c5e9.png" } } ], "cimVersion": "2.0.0" } ` If a symbol is defined, it is placed in a folder based on the type(gltf,jpeg,png) and given a symbolLayer json. The symbolLayer json is named based on the unique symbol name, and the resource property in the symbolLayer json is an href to an image or glb file. The supported symbol resource types are JPEG, PNG, glb.gz. The glb file type is a binary representation of 3D models saved in the gltf, then compressed with gzip. Example of the resource symbolLayer json ` { "name": "5fe9e487e2230d61de71aff13744c5e9", "type": "PointSymbol3D", "symbolLayers": [ { "type": "Object", "anchorPosition": [ 0, 0, -0.5 ], "width": 26.685164171278601, "height": 20, "depth": 64.389789603982777, "heading": -90, "anchor": "relative", "resource": { "href": "./resource/5fe9e487e2230d61de71aff13744c5e9.glb.gz" } } ] } `
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesPsl {}
///The 3DSceneLayerInfo object describes the properties of a layer in a store. Every scene layer contains 3DSceneLayerInfo. For features based scene layers, such as 3D objects or point scene layers, may include the default symbology, as specified in the drawingInfo, which contains stylization information for a feature layer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneLayerInfoPsl {
    ///ZFactor to define conversion factor for elevation unit.
    #[serde(rename = "ZFactor", default, skip_serializing_if = "Option::is_none")]
    pub z_factor: Option<f64>,
    ///The display alias to be used for this layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<Box<str>>,
    ///Provides the schema and layout used for storing attribute content in binary format in I3S.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub attribute_storage_info: Vec<AttributeStorageInfo>,
    ///Indicates if any styling information represented as drawingInfo is captured as part of the binary mesh representation. This helps provide optimal client-side access. Currently the color component of the drawingInfo is supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cached_drawing_info: Option<CachedDrawingInfo>,
    ///Capabilities supported by this layer. Possible values for each array string: `View`: View is supported. `Query`: Query is supported. `Edit`: Edit is defined. `Extract`: Extract is defined.
    pub capabilities: Vec<Capabilities>,
    ///Copyright and usage information for the data in this layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copyright_text: Option<Box<str>>,
    ///Description string for this layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    ///Indicates if client application will show the popup information. Default is FALSE.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_popup: Option<bool>,
    ///An object containing drawing information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drawing_info: Option<DrawingInfo>,
    ///An object containing elevation drawing information. If absent, any content of the scene layer is drawn at its z coordinate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elevation_info: Option<ElevationInfo>,
    ///A collection of objects that describe each attribute field regarding its field name, datatype, and a user friendly name {name,type,alias}. It includes all fields that are included as part of the scene layer as derived from a source input feature layer.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub fields: Vec<Field>,
    ///3D extent. If `layer.fullExtent.spatialReference` is specified, it must match `layer.spatialReference`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_extent: Option<FullExtent>,
    ///Define the layouts of point geometry and its attributes.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub geometry_definitions: Vec<GeometryDefinitionPsl>,
    ///Enables consuming clients to quickly determine whether this layer is compatible (with respect to its horizontal and vertical coordinate system) with existing content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height_model_info: Option<HeightModelInfo>,
    ///The relative URL to the 3DSceneLayerResource. Only present as part of the SceneServiceInfo resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<Box<str>>,
    ///Unique numeric ID of the layer.
    pub id: u32,
    ///The user-visible layer type. Must be: `Point`
    pub layer_type: LayerType,
    ///The name of this layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    ///The paged-access index description. For legacy purposes, this property is called pointNodePages in [Point Scene Layers](3DSceneLayer.psl.md).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_node_pages: Option<NodePageDefinition>,
    ///PopupInfo of the scene layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popup_info: Option<PopupInfo>,
    ///Range info is used to filter features of a layer withing a min and max range. The min and max range is created from the statistical information of the range field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_info: Option<RangeInfo>,
    ///The time of the last update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_update_time_stamp: Option<ServiceUpdateTimeStamp>,
    ///The spatialReference of the layer including the vertical coordinate reference system (CRS). Well Known Text (WKT) for CRS is included to support custom CRS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spatial_reference: Option<SpatialReference>,
    ///Contains the statistical information for a layer.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub statistics_info: Vec<StatisticsInfo>,
    ///The store object describes the exact physical storage of a layer and enables the client to detect when multiple layers are served from the same store.
    pub store: StorePsl,
    ///Time info represents the temporal data of a time-aware layer. The time info class provides information such as date fields that store the start and end times for each feature and the total time span for the layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_info: Option<TimeInfo>,
    ///The ID of the last update session in which any resource belonging to this layer has been updated.
    pub version: Box<str>,
}
///The store object describes the exact physical storage of a layer and enables the client to detect when multiple layers are served from the same store. Storing multiple layers in a single store - and thus having them share resources - enables efficient serving of many layers of the same content type, but with different attribute schemas or different symbology applied.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorePsl {
    ///MIME type for the encoding used for the Attribute Resources. For example: application/octet-stream; version=1.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribute_encoding: Option<Box<str>>,
    ///A common, global ArrayBufferView definition that can be used if the schema of vertex attributes and face attributes is consistent in an entire cache; this is a requirement for meshpyramids caches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_geometry_schema: Option<DefaultGeometrySchema>,
    ///If a store uses only one material, it can be defined here entirely as a MaterialDefinition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_material_definition: Option<MaterialDefinition>,
    ///A common, global TextureDefinition to be used for all textures in this store. The default texture definition uses a reduced profile of the full TextureDefinition, with the following attributes being mandatory: encoding, uvSet, wrap and channels.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub default_texture_definition: Vec<Texture>,
    ///The 2D spatial extent (xmin, ymin, xmax, ymax) of this store, in the horizontal indexCRS.
    #[serde(default)]
    pub extent: [f64; 4],
    ///MIME type for the encoding used for the Feature Data Resources. For example: application/vnd.esri. I3S.json+gzip; version=1.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_encoding: Option<Box<str>>,
    ///MIME type for the encoding used for the Geometry Resources. For example: application/octet-stream; version=1.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geometry_encoding: Option<Box<str>>,
    ///A store ID, unique across a SceneServer. Enables the client to discover which layers are part of a common store, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<str>>,
    ///The horizontal CRS used for all minimum bounding spheres (mbs) in this store. The CRS is identified by an OGC URL. Needs to be identical to the spatial reference.
    #[serde(rename = "indexCRS", default, skip_serializing_if = "Option::is_none")]
    pub index_crs: Option<Box<str>>,
    ///Information on the Indexing Scheme (QuadTree, R-Tree, Octree, ...) used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_scheme: Option<Box<str>>,
    ///Optional field to indicate the [LoD switching](lodSelection.cmn.md) mode. Possible values are: `node-switching`: A parent node is substituted for its children nodes when its lod threshold is exceeded. This implies that: parent and children are never shown at the same time. The bounding volumne of the parent has to enclose the features of all grandchildren. Nodes have a single parent, except the root node that have no parent. `none`: No switching model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod_model: Option<LodModel>,
    ///Optional field to indicate which LoD generation scheme is used in this store. Possible values are: `MeshPyramid`: Used for integrated mesh and 3D scene layer. `AutoThinning`: Used for point scene layer. `Clustering`: Used for point cloud scene layer`Generalizing`: Used for point cloud scene layer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod_type: Option<LodType>,
    ///MIME type for the encoding used for the Node Index Documents. Example: application/vnd.esri. I3S.json+gzip; version=1.6.
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
    ///MIME type(s) for the encoding used for the Texture Resources.
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

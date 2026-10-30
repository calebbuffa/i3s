#[allow(unused_imports)]
use super::*;
///The 3DSceneLayerInfo describes the properties of a layer in a store. The store object describes the exact physical storage of a layer and enables the client to detect when multiple layers are served from the same store. Every scene layer contains 3DSceneLayerInfo. If features based scene layers, such as 3D objects or point scene layers, may include the default symbology. This is as specified in the drawingInfo, which contains styling information for a feature layer. When generating 3D Objects or Integrated Mesh scene layers, the root node never has any geometry. Any node's children represent a higher LoD quality than an ancestor node. Nodes without geometry at the top of the tree are allowable since the lowest LoD of a feature/geometry is not to shown.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneLayerInfo {
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
    ///Define the layouts of mesh geometry and its attributes.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub geometry_definitions: Vec<GeometryDefinition>,
    ///Enables consuming clients to quickly determine whether this layer is compatible (with respect to its horizontal and vertical coordinate system) with existing content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height_model_info: Option<HeightModelInfo>,
    ///The relative URL to the 3DSceneLayerResource. Only present as part of the SceneServiceInfo resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<Box<str>>,
    ///Unique numeric ID of the layer.
    pub id: u32,
    ///The user-visible layer typePossible values are: `3DObject`IntegratedMesh`
    pub layer_type: LayerType,
    ///List of materials classes used in this layer.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub material_definitions: Vec<MaterialDefinitions>,
    ///The name of this layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    ///The paged-access index description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_pages: Option<NodePageDefinition>,
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
    pub store: Store,
    ///Defines the set of textures that can be referenced by meshes.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub texture_set_definitions: Vec<TextureSetDefinition>,
    ///Time info represents the temporal data of a time-aware layer. The time info class provides information such as date fields that store the start and end times for each feature and the total time span for the layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_info: Option<TimeInfo>,
    ///The ID of the last update session in which any resource belonging to this layer has been updated.
    pub version: Box<str>,
}

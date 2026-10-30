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
///Semantics for work discipline groups which can be used to refine the user experience. Possible values are: `Mechanical`Architectural`Piping`Electrical`Structural`Infrastructure`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Discipline {
    #[serde(rename = "Mechanical")]
    Mechanical,
    #[serde(rename = "Architectural")]
    Architectural,
    #[serde(rename = "Piping")]
    Piping,
    #[serde(rename = "Electrical")]
    Electrical,
    #[serde(rename = "Structural")]
    Structural,
    #[serde(rename = "Infrastructure")]
    Infrastructure,
}
impl Default for Discipline {
    fn default() -> Self {
        Self::Mechanical
    }
}
///Declares type or filter authoring info. Must be: `checkbox`: Client UI with checkbox representation for each filter type and filter value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FilterAuthoringInfoBldType {
    #[serde(rename = "checkbox")]
    Checkbox,
}
impl Default for FilterAuthoringInfoBldType {
    fn default() -> Self {
        Self::Checkbox
    }
}
///Declares filter mode of type solid. Must be: `solid`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FilterModeSolidBldType {
    #[serde(rename = "solid")]
    Solid,
}
impl Default for FilterModeSolidBldType {
    fn default() -> Self {
        Self::Solid
    }
}
///Declares filter mode of type wire frame. Must be: `wireFrame`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FilterModeWireFrameBldType {
    #[serde(rename = "wireFrame")]
    WireFrame,
}
impl Default for FilterModeWireFrameBldType {
    fn default() -> Self {
        Self::WireFrame
    }
}
///Must be: `Building`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayerBldLayerType {
    #[serde(rename = "Building")]
    Building,
}
impl Default for LayerBldLayerType {
    fn default() -> Self {
        Self::Building
    }
}
///A fixed string of building information, similar to a filter. Used by client applications to define specific behavior for the modelName. The [default filter types](./defaultFilterTypes.bld.md) define the modelName for the attribute statistics. Possible values are: `category`family`familyType`bldgLevel`createdPhase`demolishedPhase`discipline`assemblyCode`omniClass`systemClassifications`systemType`systemName`systemClass`custom`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelName {
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "family")]
    Family,
    #[serde(rename = "familyType")]
    FamilyType,
    #[serde(rename = "bldgLevel")]
    BldgLevel,
    #[serde(rename = "createdPhase")]
    CreatedPhase,
    #[serde(rename = "demolishedPhase")]
    DemolishedPhase,
    #[serde(rename = "discipline")]
    Discipline,
    #[serde(rename = "assemblyCode")]
    AssemblyCode,
    #[serde(rename = "omniClass")]
    OmniClass,
    #[serde(rename = "systemClassifications")]
    SystemClassifications,
    #[serde(rename = "systemType")]
    SystemType,
    #[serde(rename = "systemName")]
    SystemName,
    #[serde(rename = "systemClass")]
    SystemClass,
    #[serde(rename = "custom")]
    Custom,
}
impl Default for ModelName {
    fn default() -> Self {
        Self::Category
    }
}
///Possible values are: `group`3DObject`Point`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SublayerBldLayerType {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "3DObject")]
    ThreeDObject,
    #[serde(rename = "Point")]
    Point,
}
impl Default for SublayerBldLayerType {
    fn default() -> Self {
        Self::Group
    }
}
///Concatenated attribute statistics. If needed, the type of the attribute (string or number) may be inferred from `mostFrequentValues` and/or `min`/`max` fields.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeStatisticsBld {
    ///Name of the field.
    pub field_name: Box<str>,
    ///Label of the field name. If label is empty, the label and fieldName are identical.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<str>>,
    ///Maximum value. Numeric attributes only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    ///Minimum value. Numeric attributes only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    ///A fixed string of building information, similar to a filter. Used by client applications to define specific behavior for the modelName. The [default filter types](./defaultFilterTypes.bld.md) define the modelName for the attribute statistics. Possible values are: `category`family`familyType`bldgLevel`createdPhase`demolishedPhase`discipline`assemblyCode`omniClass`systemClassifications`systemType`systemName`systemClass`custom`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<ModelName>,
    ///Most frequent value, if applicable for this attribute. Truncated to 256 entries.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub most_frequent_values: Vec<u32>,
    ///List of sublayers where this attribute may be found.
    pub sub_layer_ids: Vec<u32>,
}
///Building scene layers can be filtered by field types in the building category layer. These predefined filter types are always included in the statistical information of the building scene layer. Some filter types are common to all buildings. In addition to the default filters, other fields can be included by setting the modelName to custom. Filter types are used in the buildings [statistical information](attributestats.bld.md) as well as in the [filter authoring info](filterAuthoringInfo.bld.md). The following list contains all default filter types and modelNames when creating a building scene layer using ArcGIS.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultFilterTypesBld {}
///The filter authoring info object contains metadata about the authoring process for creating a filter object. This allows the authoring client to save specific, overridable settings. The next time it is accessed with an authoring client, the selections are remembered. Non-authoring clients can ignore it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterAuthoringInfoBld {
    ///Array of filter block authoring info.
    pub filter_blocks: Vec<FilterBlockAuthoringInfoBld>,
    ///Declares type or filter authoring info. Must be: `checkbox`: Client UI with checkbox representation for each filter type and filter value.
    pub r#type: FilterAuthoringInfoBldType,
}
///The filter object can be applied to a building scene layer. Filter allows client applications to reduce the drawn elements of a building to specific types and values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterBld {
    ///Description of the filter.
    pub description: Box<str>,
    ///Authoring Info used to generate user interface for authoring clients.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_authoring_info: Option<FilterAuthoringInfoBld>,
    ///Array of filter blocks defining the filter. A filter contains at least one filter block.
    pub filter_blocks: Vec<FilterBlockBld>,
    ///Global ID as unique identifier of the filter.
    pub id: Box<str>,
    ///Indicates if a filter is the default filter. Clients use the default filter to show the current state of a building. For example, if 'created' is the default filter, all elements in the 'created' phases are drawn, while elements in the 'demolished' phases are invisible. The default filter is not shown in the UI and does not have Authoring Info. (Can build specific UI for this filter)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default_filter: Option<bool>,
    ///Defines if a filter is visible within the client application. Used to exclude filters that are overwritten from a group of filters shown in the client application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,
    ///Name of the filter.
    pub name: Box<str>,
}
///The filter authoring info object contains metadata about the authoring process for creating a filter block object. This allows the authoring client to save specific, overridable settings. The next time it is accessed via an authoring client, their selections are remembered. Non-authoring clients can ignore it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterBlockAuthoringInfoBld {
    ///Array of defined filter types. Each filter type has an array of filter values.
    pub filter_types: Vec<FilterTypeBld>,
}
///A filter block defines what elements will be filtered with a specific filter mode. To ensure performance on client applications, it is not recommended to declare multiple filter blocks with the same filter mode. Filter blocks are contained in a filter for a building scene layer. Each filter includes at least one filter block.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterBlockBld {
    ///Filter query expression for a building scene layer.
    pub filter_expression: Box<str>,
    ///Filter mode defines how features are drawn. For example, the filter mode of a filter can be solid or wire frame.
    pub filter_mode: FilterModeBld,
    ///Title of the filter block.
    pub title: Box<str>,
}
///Filter mode represents the way elements draw when participating in a filter block.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterModeBld {}
///Shows all elements that comply with the filter block of a filter in a building scene layer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterModeSolidBld {
    ///Declares filter mode of type solid. Must be: `solid`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FilterModeSolidBldType>,
}
///Shows all elements that comply with the filter block of a filter in a building scene layer. The elements are drawn with an edge line.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterModeWireFrameBld {
    ///An object defining solid edges of a feature. [See more](https://developers.arcgis.com/web-scene-specification/objects/edges/) information on supported edges in ArcGIS clients.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<serde_json::Value>,
    ///Declares filter mode of type wire frame. Must be: `wireFrame`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FilterModeWireFrameBldType>,
}
///The file authoring information for a filter, including the filter type and its value settings.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterTypeBld {
    ///Represents the filter type name. Name is a unique identifier.
    pub filter_type: Box<str>,
    ///Array of filter values. Filter values are the attributes that can be stored for individual fields in a layer.
    pub filter_values: Vec<Box<str>>,
}
///#### Building scene layer structure A building scene layer represents a 3D BIM model as a single layer composed of sublayers. The 3D BIM model can be any man made structure organized in discipline layers (groups) such as Architectural, Electrical, Infrastructure, Mechanical, Piping or Structural and category layers representing content such as walls or windows. A building scene layer may contain an overview including only exterior feature in addition to the full model. The concept of a group (i.e. `layerType='group'`) has been added to organized sublayers into a nested tree structure that can be reflected in the table of content of 3D Clients. If a building scene layer does not contain an overview, the structure should not include an overview or full model, only the disciplines directly. Please note that: - Groups and sublayers may be referenced **once** (e.g. a sublayer may not be in multiple groups). - Groups and sublayers do not have any resource associated with them. - Sublayer resources are located in the sublayers of the building scene layer: layers/{bim_layer_id}/sublayers/{sub_layer_id}/.... Since a building scene layer may have an associated featureService, care must be taken to match building scene layer sublayer IDs with the service. In practice, if the building scene layer has n sublayers numbered \[0,n-1\] they need to match the featureService sublayers IDs. Any group layers ID in the scene layer need to be greater. ` +-- layers | +-- 10 (3dSceneLayer.json for layer10, layerType ='building' ) | | +-- statistics | | | +-- summary.json | | +-- sublayers | | | +--0 (3dSceneLayer.json for layer0, layerType='3DObject') | | | | +--nodes | | | | | +--0 | | | | | | +--3dNodeIndexDocument.json | | | | | | +--geometries (...) | | | | | | +--attributes (...) | | | | | +--1 | | | | | | +--3dNodeIndexDocument.json | | | | | | +--geometries (...) | | | | | | +--attributes (...) | | | | | +--(...) | | | | +--statistics | | | +--1 (3dSceneLayer.json for layer1, layerType='3DObject') | | | | +-- (...) | | | +--(... , layerType='3DObject') ` #### Building scene layer service: The service definition is identical to other scene layer service definitions and will list a single layer (the BIM layer) e.g: ` js { "serviceName" : "Esri Campus", "serviceVersion" : "1.8" "supportedBindings" : "REST" "layers": [ { "id" : 10, "layerType" : "Building" // ... // building scene layer JSON definitions (see example below) // ... } ] } ` **Edits** - Group/layer names **must be unique**. - Capabilities that have been removed. - `sublayers.href` and `groups.href` have been removed in favor of IDs. - Removed `fullExtent` from `group` object. - Added back `modelName`. - Added statistics.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerBld {
    ///Global ID, filter ID of the currently active filter for the building scene layer.
    #[serde(
        rename = "activeFilterID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub active_filter_id: Option<Box<str>>,
    ///Alias of the layer name. Can be empty if alias and name are identical.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<Box<str>>,
    ///Capabilities supported by building scene layer. Overwrites any capabilities on sublayers. Possible values for each array string: `View`: View is supported. `Query`: Query is supported. `Edit`: Edit is defined. `Extract`: Extract is defined.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub capabilities: Vec<Capabilities>,
    ///Copyright information to be displayed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copyright_text: Option<Box<str>>,
    ///Description for the layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    ///Array of filters defined for the building scene layer.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub filters: Vec<FilterBld>,
    ///3d extent. If `layer.fullExtent.spatialReference` is specified, it **must** match `layer.spatialReference`.
    pub full_extent: FullExtent,
    ///An object containing the vertical coordinate system information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height_model_info: Option<HeightModelInfo>,
    ///Identifier for the layer. Building scene layer id is not in the same namespace as sublayer id. **Important**: clients should **not** assume it will be `0`.
    pub id: u32,
    ///Must be: `Building`
    pub layer_type: LayerBldLayerType,
    ///Layer name.
    pub name: Box<str>,
    ///The spatialReference of the layer including the vertical coordinate system. WKT is included to support custom spatial references.
    pub spatial_reference: SpatialReference,
    ///url to statistic summary for the BIM layer. [statistics/summary.json](attributestats.bld.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics_h_ref: Option<Box<str>>,
    ///List of sublayers or group of sublayers.
    pub sublayers: Vec<SublayerBld>,
    ///Version of building scene layer.
    pub version: Box<str>,
}
///Statistics for all building scene layer sublayers. Captures statistical information for each field in the building scene layer and the sublayers containing this fields.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsBld {
    ///Per-attribute statistics for all sublayers.
    pub summary: Vec<AttributeStatisticsBld>,
}
///Sublayer of a building scene layer. A building scene layer is composed of an overview and the full model containing the discipline and category layers. These layer types are represented as sublayers. A sublayer may contain other layers or sublayers (i.e. `group`) to form a nested structure. The order of the layers is inverted, meaning the first layer is on the bottom and the last layer is on the top.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SublayerBld {
    ///Alias of the layer name. Can be empty if alias and name are identical.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<Box<str>>,
    ///Semantics for work discipline groups which can be used to refine the user experience. Possible values are: `Mechanical`Architectural`Piping`Electrical`Structural`Infrastructure`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discipline: Option<Discipline>,
    ///Identifier for this sublayer. **If** `layerType != 'group'`, resources will be at `/layers/{bim_layer_id}/sublayers/{this.id}/...`
    pub id: u32,
    ///Returns true if the layer has no features.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    ///Possible values are: `group`3DObject`Point`
    pub layer_type: SublayerBldLayerType,
    ///A fixed string of sublayer information. Used by client applications to define specific behavior for the modelName. See [list of defined modelNames](subLayerModelName.md) for sublayers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<Box<str>>,
    ///Layer name. **Must be unique** per building scene layer.
    pub name: Box<str>,
    ///Sublayers contained in this layer.
    #[serde(
        default,
        skip_serializing_if = "is_empty_collection",
        deserialize_with = "deserialize_null_default"
    )]
    pub sublayers: Vec<SublayerBld>,
    ///Visibility of the sublayer. Default is `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<bool>,
}

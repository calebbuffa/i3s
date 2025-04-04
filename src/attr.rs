//! I3S Attribution.

use serde::{Deserialize, Serialize};

/// Supported Geometry Attributes
pub const SUPPORTED_GEOMETRY_ATTRIBUTES: &[&str] = &[
    "position",
    "normal",
    "uv0",
    "color",
    "uv-region",
    "feature-index",
];

/// Default Geometry Schema Header
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultGeometrySchemaHeader {
    pub property: String,
    #[serde(rename = "type", default)]
    pub dtype: Option<String>,
}

/// Attribute Metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeMetadata {
    #[serde(rename = "type", default)]
    pub dtype: Option<String>,
    #[serde(rename = "valuesPerElement", default)]
    pub values_per_element: Option<u32>,
    #[serde(rename = "byteOffset", default)]
    pub byte_offset: Option<u32>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub component: Option<u32>,
    #[serde(default)]
    pub binding: Option<String>,
}

/// Normal Attribute Metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NormalAttributeMetadata {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: u32,
    pub encoding: String,
    pub binding: String,
}

impl Default for NormalAttributeMetadata {
    fn default() -> Self {
        Self {
            dtype: "Float32".to_string(),
            component: 3,
            encoding: "none".to_string(),
            binding: "per-vertex".to_string(),
        }
    }
}

/// UV0 Attribute Metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UV0AttributeMetadata {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: u32,
    pub encoding: String,
    pub binding: String,
}

impl Default for UV0AttributeMetadata {
    fn default() -> Self {
        Self {
            dtype: "Float32".to_string(),
            component: 2,
            encoding: "none".to_string(),
            binding: "per-vertex".to_string(),
        }
    }
}

/// Color Attribute Metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorAttributeMetadata {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: u32,
    pub encoding: String,
    pub binding: String,
}

impl Default for ColorAttributeMetadata {
    fn default() -> Self {
        Self {
            dtype: "UInt8".to_string(),
            component: 3,
            encoding: "none".to_string(),
            binding: "per-vertex".to_string(),
        }
    }
}

/// UV Region Attribute Metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UVRegionAttributeMetadata {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: u32,
    pub encoding: String,
    pub binding: String,
}

impl Default for UVRegionAttributeMetadata {
    fn default() -> Self {
        Self {
            dtype: "UInt16".to_string(),
            component: 4,
            encoding: "none".to_string(),
            binding: "per-vertex".to_string(),
        }
    }
}

/// Face Range Attribute Metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FaceRangeAttributeMetadata {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: u32,
    pub encoding: String,
    pub binding: String,
}

impl Default for FaceRangeAttributeMetadata {
    fn default() -> Self {
        Self {
            dtype: "UInt32".to_string(),
            component: 1,
            encoding: "none".to_string(),
            binding: "per-vertex".to_string(),
        }
    }
}

/// Compressed Attributes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompressedAttributes {
    pub encoding: String,
    pub attributes: Vec<String>,
}

impl CompressedAttributes {
    pub fn validate(&self) -> bool {
        if self.encoding != "draco" {
            return false;
        }
        self.attributes
            .iter()
            .all(|attr| SUPPORTED_GEOMETRY_ATTRIBUTES.contains(&attr.as_str()))
    }
}

/// Attribute Storage Info
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeStorageInfo {
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub ordering: Option<Vec<String>>,
    #[serde(rename = "attributeValues", default)]
    pub attribute_values: Option<Vec<AttributeMetadata>>,
    #[serde(default)]
    pub header: Option<Vec<DefaultGeometrySchemaHeader>>,
    #[serde(rename = "attributeByteCountsList", default)]
    pub attribute_byte_countslist: Option<AttributeMetadata>,
}

/// Domain Coded Value
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DomainCodedValue {
    pub name: String,
    pub code: String,
}

/// Domain
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub description: String,
    #[serde(rename = "fieldType")]
    pub field_type: String,
    #[serde(rename = "type", default)]
    pub dtype: Option<String>,
    #[serde(rename = "codedValues", default)]
    pub coded_values: Option<Vec<DomainCodedValue>>,
}

/// Attribute Field
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub alias: String,
    #[serde(default)]
    pub domain: Option<Domain>,
}

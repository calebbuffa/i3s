//! Enum Options.

use serde::{Deserialize, Serialize};

/// Compression options
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Compression {
    Uncompressed = 0,
    Compressed = 1,
}

/// Layer types supported by the I3S specification
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum LayerType {
    IntegratedMesh,
    DDDObject,
    Point,
    Building,
}

/// Geometry bindings supported by the I3S specification
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum GeometryBinding {
    PerVertex,
    PerUVRegion,
}

/// Data types supported by the I3S specification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataType {
    Float32,
    Float64,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    String,
    OID32,
}

impl DataType {
    /// Get the data type from a string
    pub fn from_str(data_type: &str) -> Option<Self> {
        match data_type {
            "Float32" => Some(DataType::Float32),
            "Float64" => Some(DataType::Float64),
            "Int16" => Some(DataType::Int16),
            "Int32" => Some(DataType::Int32),
            "Int64" => Some(DataType::Int64),
            "UInt8" => Some(DataType::UInt8),
            "UInt16" => Some(DataType::UInt16),
            "UInt32" => Some(DataType::UInt32),
            "UInt64" => Some(DataType::UInt64),
            "String" => Some(DataType::String),
            "OID32" => Some(DataType::OID32),
            _ => None,
        }
    }
}

/// Ordering supported by the I3S specification
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Ordering {
    AttributeByteCounts,
    AttributeValues,
}

/// Esri Field Types
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum FieldType {
    Double,
    Integer,
    OID,
    String,
}

/// Texture Formats
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all="lowercase")]
pub enum ImageFormat {
    PNG,
    JPG,
    DDS,
    KTX2,
    KtcEtc2,
}

impl AsRef<str> for ImageFormat {
    fn as_ref(&self) -> &str {
        match self {
            ImageFormat::PNG => "png",
            ImageFormat::JPG => "jpg",
            _ => todo!(),
        }
    }
}

/// LOD Selection Metrics
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LODSelectionMetric {
    MaxScreenThresholdSQ,
    DensityThreshold,
}

/// Resource Patterns
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ResourcePattern {
    #[serde(rename = "3dNodeIndexDocument")]
    NodeIndexDocument,
    SharedResource,
    FeatureData,
    Geometry,
    Texture,
    Attributes,
}

/// Normal Reference Frames
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum NormalReferenceFrame {
    EastNorthUp,
    EarthCentered,
    VertexReferenceFrame,
}

/// LOD Types
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum LODType {
    MeshPyramid,
    AutoThinning,
    Clustering,
    Generalizing,
}

/// Geometry Types
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum GeometryType {
    Triangles,
}

/// Topology Types
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Topology {
    PerAttributeArray,
    Indexed,
}

/// Profiles
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Profile {
    MeshPyramids,
    Points,
    PointClouds,
    Building,
}

/// I3S Format
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum I3SFormat {
    SLPK,
    REST,
}

impl I3SFormat {
    /// Determine the format from a URI
    pub fn from_uri(uri: &str) -> Result<Self, String> {
        if uri.ends_with(".slpk") {
            Ok(I3SFormat::SLPK)
        } else if uri.starts_with("http") {
            Ok(I3SFormat::REST)
        } else {
            Err("Invalid URI".to_string())
        }
    }
}

/// Capabilities
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Capabilities {
    View,
    Edit,
    Query,
}

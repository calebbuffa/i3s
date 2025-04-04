//! Geometry I/O for I3S format.

use crate::attr::{AttributeMetadata, CompressedAttributes};
use crate::options::{GeometryType, Topology};
use serde::{Deserialize, Serialize};

/// Geometry Buffer
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeometryBuffer {
    #[serde(rename = "compressedAttributes", default)]
    pub compressed_attributes: Option<CompressedAttributes>,
    pub offset: Option<i32>,
    pub position: Option<AttributeMetadata>,
    pub normal: Option<AttributeMetadata>,
    pub uv0: Option<AttributeMetadata>,
    pub color: Option<AttributeMetadata>,
    #[serde(rename = "featureId", default)]
    pub feature_id: Option<AttributeMetadata>,
    #[serde(rename = "faceRange", default)]
    pub face_range: Option<AttributeMetadata>,
    #[serde(rename = "uvRegion", default)]
    pub uv_region: Option<AttributeMetadata>,
}

impl Default for GeometryBuffer {
    fn default() -> Self {
        Self {
            compressed_attributes: None,
            offset: None,
            position: None,
            normal: None,
            uv0: None,
            color: None,
            feature_id: None,
            face_range: None,
            uv_region: None,
        }
    }
}

/// Geometry Definition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeometryDefinition {
    #[serde(rename = "geometryBuffers")]
    pub geometry_buffers: Vec<GeometryBuffer>,
    pub topology: Option<String>,
}

impl GeometryDefinition {
    /// Check if the geometry definition has compressed attributes.
    pub fn has_compressed(&self) -> bool {
        self.geometry_buffers
            .iter()
            .any(|gb| gb.compressed_attributes.is_some())
    }

    /// Get the compressed geometry buffers.
    pub fn compressed_geometry_buffers(&self) -> Vec<&GeometryBuffer> {
        self.geometry_buffers
            .iter()
            .filter(|gb| gb.compressed_attributes.is_some())
            .collect()
    }

    /// Get the uncompressed geometry buffers.
    pub fn uncompressed_geometry_buffers(&self) -> Vec<&GeometryBuffer> {
        self.geometry_buffers
            .iter()
            .filter(|gb| gb.compressed_attributes.is_none())
            .collect()
    }
}

/// Default Geometry Schema
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DefaultGeometrySchema {
    pub topology: Topology,
    #[serde(rename = "geometryType", default = "default_geometry_type")]
    pub geometry_type: GeometryType,
    pub ordering: Vec<String>,
    pub header: Vec<serde_json::Value>,
}

fn default_geometry_type() -> GeometryType {
    GeometryType::Triangles
}

//! Mesh data.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Mesh Material
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeshMaterial {
    pub definition: usize,
    pub resource: usize,
    #[serde(rename = "texelCountHint", default)]
    pub texel_count_hint: Option<usize>,
    #[serde(skip)]
    pub(crate) cache: HashMap<String, Arc<Vec<u8>>>,
}

/// Mesh Geometry
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeshGeometry {
    pub definition: usize,
    pub resource: usize,
    #[serde(rename = "vertexCount")]
    pub vertex_count: usize,
    #[serde(rename = "featureCount", default)]
    pub feature_count: Option<usize>,
    #[serde(skip)]
    pub(crate) cache: HashMap<String, Arc<Vec<u8>>>,
}

/// Mesh Attribute
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeshAttribute {
    pub resource: i32,
}

/// Mesh
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mesh {
    pub geometry: MeshGeometry,
    #[serde(default)]
    pub material: Option<MeshMaterial>,
    #[serde(default)]
    pub attribute: Option<MeshAttribute>,
}

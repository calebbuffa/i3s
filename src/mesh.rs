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

// impl Default for MeshMaterial {
//     fn default() -> Self {
//         Self {
//             definition: 0,
//             resource: 0,
//             texel_count_hint: None,
//             // cache: HashMap::new(),
//         }
//     }
// }

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
    #[serde(skip)]
    pub(crate) cache: HashMap<String, Arc<serde_json::Value>>,
}

// impl Default for MeshAttribute {
//     fn default() -> Self {
//         Self {
//             resource: 0,
//             // cache: HashMap::new(),
//         }
//     }
// }

/// Mesh
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mesh {
    pub geometry: MeshGeometry,
    #[serde(default)]
    pub material: Option<MeshMaterial>,
    #[serde(default)]
    pub attribute: Option<MeshAttribute>,
    // #[serde(skip)]
    // pub(crate) cache: HashMap<String, Arc<serde_json::Value>>,
}

// impl Default for Mesh {
//     fn default() -> Self {
//         Self {
//             geometry: MeshGeometry::default(),
//             material: None,
//             attribute: None,
//             // cache: HashMap::new(),
//         }
//     }
// }

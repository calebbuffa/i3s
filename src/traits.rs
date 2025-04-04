use crate::options::Compression;
use crate::mesh::{MeshGeometry, MeshMaterial};
use std::sync::Arc;

use crate::node::Node;

pub(crate) trait UriBuilder {
    fn create_geometry_uri(
        &self,
        resource: &usize,
        compression: &Compression,
    ) -> Option<String>;
    fn create_texture_uri(
        &self,
        resource: &usize,
        name: &str,
        fmt: &str,
        compression: &Compression,
    ) -> Option<String>;
}

/// Resource Manager Protocol
pub(crate) trait Accessor {
    fn get_node(&self, index: &usize) -> Result<Node, String>;
    fn get(&self, uri: &str) -> Result<Vec<u8>, String>;
}

/// Decoder trait
pub(crate) trait Decoder {
    fn decode_geometry(
        &self,
        geometry: &mut MeshGeometry,
        compression: &Compression,
    ) -> Result<Arc<Vec<u8>>, String>;

    fn decode_material(
        &self,
        material: &mut MeshMaterial,
        compression: &Compression,
    ) -> Result<Arc<Vec<u8>>, String>;
}

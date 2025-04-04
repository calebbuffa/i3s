//! Resource Managers.

use crate::accessor::Accessor;
use crate::defn::SceneDefinition;
use crate::node::{Node, NodePage, get_node_index_in_node_page, get_node_page_index};
use crate::options::Compression;
use crate::uri::UriBuilder;
use dashmap::DashMap;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};
use zip::ZipArchive;

fn decode_scene_definition(data: &Vec<u8>) -> Result<SceneDefinition, String> {
    let mut decoder = GzDecoder::new(&data[..]);
    let mut decompressed = Vec::new();
    let decoded = decoder.read_to_end(&mut decompressed);
    if decoded.is_ok() {
        if let Ok(json) = serde_json::from_slice(&decompressed) {
            return Ok(json);
        } else {
            return Err("Failed to parse 3dSceneLayer.json.gz".to_string());
        }
    } else {
        return Err("Failed to decompress 3dSceneLayer.json.gz".to_string());
    }
}
fn get_data_from_locked_zip(
    archive: &mut RwLock<ZipArchive<File>>,
    uri: &str,
) -> Result<Vec<u8>, String> {
    let mut archive = archive.write().unwrap();
    let mut file = archive
        .by_name(uri)
        .map_err(|e| format!("Failed to find file in archive: {}", e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read file from archive: {}", e))?;
    Ok(buffer)
}

fn get_data_from_zip(archive: &mut ZipArchive<File>, uri: &str) -> Result<Vec<u8>, String> {
    let mut file = archive
        .by_name(uri)
        .map_err(|e| format!("Failed to find file in archive: {}", e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read file from archive: {}", e))?;
    Ok(buffer)
}

/// Scene Layer Package
pub struct SceneLayerPackage {
    archive: RwLock<ZipArchive<File>>,
    cache: DashMap<String, Arc<NodePage>>,
    pub(crate) scene_definition: SceneDefinition,
}

impl SceneLayerPackage {
    /// Get a node page by its index.
    pub fn get_node_page(&self, index: &usize) -> Result<Arc<NodePage>, String> {
        let key = format!("{}", index).to_string();
        if !self.cache.contains_key(&key) {
            let path = format!("nodepages/{}.json.gz", index);
            let compressed_data = self.get(&path)?;
            let decompressed_data = flate2::read::GzDecoder::new(&compressed_data[..])
                .bytes()
                .collect::<Result<Vec<u8>, _>>()
                .map_err(|e| format!("Failed to decompress node page data: {}", e))?;
            let node_page: NodePage = serde_json::from_slice(&decompressed_data)
                .map_err(|e| format!("Could not parse Node Page: {}", e))?;
            self.cache.insert(key.clone(), Arc::new(node_page));
        }
        let node_page = self.cache.get(&key).unwrap().clone();
        Ok(node_page)
    }

    fn compressed_texture_uri(
        &self,
        resource: &usize,
        name: &str,
        fmt: &str,
    ) -> Result<String, String> {
        let scene_definition = &self.scene_definition;
        let texture_definitions = scene_definition.texture_set_definitions.as_ref();
        if let Some(texture_definitions) = texture_definitions {
            for texture_def in texture_definitions {
                if texture_def.has_compressed() {
                    return Ok(format!(
                        "nodes/{}/textures/{}.bin.{}.gz",
                        resource, name, fmt
                    ));
                }
            }
            Err("No compressed texture URI available".to_string())
        } else {
            return Err("Texture definitions not found in scene definition.".to_string());
        }
    }

    fn uncompressed_texture_uri(
        &self,
        resource: &usize,
        name: &str,
        fmt: &str,
    ) -> Result<String, String> {
        if self.scene_definition.texture_set_definitions.is_none() {
            return Err("Texture definitions not found in scene definition.".to_string());
        }
        Ok(format!("nodes/{}/textures/{}.bin.{}", resource, name, fmt))
    }

    fn compressed_geometry_uri(&self, resource: &usize) -> Result<String, String> {
        let scene_definition = &self.scene_definition;
        let geometry_definitions = scene_definition.geometry_definitions.as_ref();
        if let Some(geometry_definitions) = geometry_definitions {
            for geometry_def in geometry_definitions {
                if geometry_def.has_compressed() {
                    return Ok(format!("nodes/{}/geometries/1.bin", resource,));
                }
            }
            Err("No compressed geometry URI available".to_string())
        } else {
            return Err("Geometry definitions not found in scene definition.".to_string());
        }
    }

    fn uncompressed_geometry_uri(&self, resource: &usize) -> Result<String, String> {
        Ok(format!("nodes/{}/geometries/0.bin", resource))
    }

    /// Create a new SceneLayerPackage from a file path.
    pub fn open(uri: &str) -> Result<SceneLayerPackage, String> {
        let file = File::open(uri).map_err(|e| format!("Failed to open file: {}", e))?;
        let mut archive =
            ZipArchive::new(file).map_err(|e| format!("Failed to read ZIP archive: {}", e))?;
        // Read the 3dSceneLayer.json.gz file from the archive.
        let data = get_data_from_zip(&mut archive, "3dSceneLayer.json.gz")?;
        // Decode the scene definition from the data.
        let scene_definition = decode_scene_definition(&data)
            .map_err(|e| format!("Failed to decode scene definition: {}", e))?;
        // Create the SceneLayerPackage instance.
        let slpk = SceneLayerPackage {
            archive: RwLock::new(archive),
            cache: DashMap::new(),
            scene_definition: scene_definition,
        };
        Ok(slpk)
    }
}

impl Accessor for SceneLayerPackage {
    fn get(&self, uri: &str) -> Result<Vec<u8>, String> {
        let mut archive = self.archive.write().unwrap();
        let mut file = archive
            .by_name(uri)
            .map_err(|e| format!("Failed to find file in archive: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file from archive: {}", e))?;
        Ok(buffer)
    }

    /// Get a node by its index.
    fn get_node(&self, index: &usize) -> Result<Arc<Node>, String> {
        // Retrieve the nodePages definition from the scene definition.
        let scene_definition = &self.scene_definition;
        let node_page_def_result = scene_definition
            .node_pages
            .as_ref()
            .ok_or("NodePages definition not found in scene definition.");
        if node_page_def_result.is_err() {
            return Err(node_page_def_result.unwrap_err().to_string());
        }
        let node_page_def = node_page_def_result.unwrap();

        // Get the number of nodes per page.
        let nodes_per_page = node_page_def.nodes_per_page;

        // Calculate the node page index and retrieve the node page.
        let node_page_index = get_node_page_index(index, &nodes_per_page);
        let node_page = self.get_node_page(&node_page_index)?;

        // Calculate the node index within the node page and retrieve the node.
        let node_index = get_node_index_in_node_page(index, &nodes_per_page);
        let num_nodes = node_page.nodes.len();
        if node_index >= num_nodes {
            return Err(format!(
                "Index {} is greater than {} nodes in the node page",
                node_index, num_nodes
            ));
        }
        let node = Arc::new(node_page.nodes[node_index].clone());
        Ok(node)
    }
}

impl UriBuilder for SceneLayerPackage {
    /// Create a geometry URI.
    fn create_geometry_uri(
        &self,
        resource: &usize,
        compression: &Compression,
    ) -> Result<String, String> {
        match compression {
            Compression::Compressed => self.compressed_geometry_uri(resource),
            Compression::Uncompressed => self.uncompressed_geometry_uri(resource),
        }
    }

    /// Create a texture URI.
    fn create_texture_uri(
        &self,
        resource: &usize,
        name: &str,
        fmt: &str,
        compression: &Compression,
    ) -> Result<String, String> {
        match compression {
            Compression::Compressed => self.compressed_texture_uri(resource, name, fmt),
            Compression::Uncompressed => self.uncompressed_texture_uri(resource, name, fmt),
        }
    }
}

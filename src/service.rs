//! Resource Managers.

use crate::defn::SceneDefinition;
use crate::node::{Node, NodePage, get_node_index_in_node_page, get_node_page_index};
use crate::options::Compression;
use crate::traits::{Accessor, UriBuilder};
use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use reqwest::blocking::Client;
use serde_json::Value;
use std::io::Read;
use std::sync::Arc;

fn get_scene_definition(client: &Client, base_url: &str) -> Result<SceneDefinition, String> {
    let url = format!("{}/layers/0", base_url);
    let response = client
        .get(&url)
        .send()
        .map_err(|e| format!("Request error: {}", e))?;
    let json: Value = response
        .json()
        .map_err(|e| format!("JSON parse error: {}", e))?;
    let scene_defn: SceneDefinition = serde_json::from_value(json)
        .map_err(|e| format!("Could not parse scene definition: {}", e))?;
    Ok(scene_defn)
}

/// Scene Layer Service
pub struct Service {
    base_url: String,
    node_pages: DashMap<usize, NodePage>,
    client: Client,
    pub(crate) scene_definition: SceneDefinition,
}

impl Service {
    /// Create a new Service from a base URL.
    pub fn connect(base_url: &str) -> Result<Self, String> {
        let base_url = if base_url.ends_with("SceneServer") {
            base_url.to_string()
        } else {
            format!("{}/SceneServer", base_url.trim_end_matches('/'))
        };

        let client = Client::new();
        let scene_definition = get_scene_definition(&client, &base_url)
            .map_err(|e| format!("Failed to get scene definition: {}", e))?;

        Ok(Service {
            base_url,
            node_pages: DashMap::new(),
            client: client,
            scene_definition: scene_definition,
        })
    }

    /// Get a node page by index.
    pub fn get_node_page(&self, index: &usize) -> Result<Ref<usize, NodePage>, String> {
        if !self.node_pages.contains_key(index) {
            let url = format!("{}/layers/0/nodepages/{}", self.base_url, index);
            let data = self
                .get(&url)
                .map_err(|e| format!("Failed to fetch data: {}", e))?;
            let json: Value =
                serde_json::from_slice(&data).map_err(|e| format!("JSON parse error: {}", e))?;
            if json.get("error").is_some() {
                return Err(json.to_string());
            }
            let node_page: NodePage =
                serde_json::from_value(json).map_err(|e| format!("Unable to parse NodePage: {}", e))?;
            self.node_pages.insert(index.clone(), node_page);
        }
        let node_page = self.node_pages.get(index).unwrap();
        Ok(node_page)
    }

    fn uncompressed_texture_uri(&self, resource: &usize, name: &str) -> Option<String> {
        let scene_definition = &self.scene_definition;
        let texture_definitions = scene_definition.texture_set_definitions.as_ref();
        if let Some(texture_definitions) = texture_definitions {
            if !texture_definitions.is_empty() {
                return Some(format!("layers/0/nodes/{}/textures/{}", resource, name,));
            }
            None
        } else {
            None
        }
    }
    fn compressed_texture_uri(&self, resource: &usize, name: &str) -> Option<String> {
        let scene_definition = &self.scene_definition;
        let texture_definitions = scene_definition.texture_set_definitions.as_ref();
        if let Some(texture_definitions) = texture_definitions {
            for texture_def in texture_definitions {
                if texture_def.has_compressed() {
                    return Some(format!("layers/0/nodes/{}/textures/{}", resource, name));
                }
            }
            None
        } else {
            None
        }
    }

    fn uncompressed_geometry_uri(&self, resource: &usize) -> String {
        format!("layers/0/nodes/{}/geometries/0", resource)
    }

    fn compressed_geometry_uri(&self, resource: &usize) -> Option<String> {
        let scene_definition = &self.scene_definition;
        let geometry_definitions = scene_definition.geometry_definitions.as_ref();
        if let Some(geometry_definitions) = geometry_definitions {
            for geometry_def in geometry_definitions {
                if geometry_def.has_compressed() {
                    return Some(format!("layers/0/nodes/{}/geometries/1", resource,));
                }
            }
            None
        } else {
            None
        }
    }
}

impl Accessor for Service {
    /// Get a node by index.
    fn get_node(&self, index: &usize) -> Result<Node, String> {
        let scene_definition = &self.scene_definition;
        let node_page_def = scene_definition
            .node_pages
            .as_ref()
            .ok_or("Node pages not found in scene definition.")?;
        let nodes_per_page = node_page_def.nodes_per_page;

        let node_page_index = get_node_page_index(index, &nodes_per_page);
        let node_page = self.get_node_page(&node_page_index)?;

        let node_index = get_node_index_in_node_page(index, &nodes_per_page);
        let num_nodes = node_page.nodes.len();
        if node_index >= num_nodes {
            return Err(format!(
                "Index {} is greater than {} nodes in the node page",
                node_index, num_nodes
            ));
        }
        let node = node_page.nodes[node_index].to_owned();
        Ok(node)
    }

    /// Fetch data from the given URI.
    fn get(&self, uri: &str) -> Result<Vec<u8>, String> {
        let url = format!("{}/{}", self.base_url, uri);
        let response = self
            .client
            .get(&url)
            .send()
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Request failed with status code: {}",
                response.status()
            ));
        }
        let mut buffer = Vec::new();
        let bytes = response
            .bytes()
            .map_err(|e| format!("Failed to read response bytes: {}", e))?;
        let mut reader = std::io::Cursor::new(bytes.as_ref());
        reader
            .read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read response: {}", e))?;
        Ok(buffer)
    }
}

impl UriBuilder for Service {
    /// Create a geometry URI.
    fn create_geometry_uri(&self, resource: &usize, compression: &Compression) -> Option<String> {
        match compression {
            Compression::Compressed => self.compressed_geometry_uri(resource),
            Compression::Uncompressed => Some(self.uncompressed_geometry_uri(resource)),
        }
    }

    /// Create a texture URI.
    fn create_texture_uri(
        &self,
        resource: &usize,
        name: &str,
        fmt: &str,
        compression: &Compression,
    ) -> Option<String> {
        let _ = fmt;
        match compression {
            Compression::Compressed => self.compressed_texture_uri(resource, name),
            Compression::Uncompressed => self.uncompressed_texture_uri(resource, name),
        }
    }
}

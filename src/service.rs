//! Resource Managers.

use crate::accessor::Accessor;
use crate::defn::SceneDefinition;
use crate::node::{Node, NodePage, get_node_index_in_node_page, get_node_page_index};
use crate::options::Compression;
use crate::uri::UriBuilder;
use dashmap::DashMap;
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
    cache: DashMap<String, Value>,
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
        let cache = DashMap::new();

        Ok(Service {
            base_url,
            cache,
            client,
            scene_definition,
        })
    }

    /// Get a node page by index.
    pub fn get_node_page(&self, index: usize) -> Result<NodePage, String> {
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
        Ok(node_page)
    }

    /// Get all node pages.
    pub fn node_pages(&self) -> Vec<NodePage> {
        let mut node_pages = Vec::new();
        let mut index = 0;
        loop {
            match self.get_node_page(index) {
                Ok(node_page) => node_pages.push(node_page),
                _ => break,
            }
            index += 1;
        }
        node_pages
    }
}

impl Accessor for Service {
    /// Get a node by index.
    fn get_node(&self, index: &usize) -> Result<Arc<Node>, String> {
        let scene_definition = &self.scene_definition;
        let node_page_def = scene_definition
            .node_pages
            .as_ref()
            .ok_or("Node pages not found in scene definition.")?;
        let nodes_per_page = node_page_def.nodes_per_page;

        let node_page_index = get_node_page_index(index, &nodes_per_page);
        let node_page = self.get_node_page(node_page_index)?;

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
    fn create_geometry_uri(
        &self,
        resource: &usize,
        compression: &Compression,
    ) -> Result<String, String> {
        let base_uri = format!("layers/0/nodes/{}/geometries", resource);
        match compression {
            Compression::Compressed => Ok(format!("{}/1", base_uri)),
            Compression::Uncompressed => Ok(format!("{}/0", base_uri)),
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
        let _ = fmt;
        match compression {
            Compression::Compressed => Ok(format!("layers/0/nodes/{}/textures/{}", resource, name)),
            Compression::Uncompressed => {
                Ok(format!("layers/0/nodes/{}/textures/{}", resource, name))
            }
        }
    }
}

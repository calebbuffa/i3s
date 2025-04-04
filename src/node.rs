//! I3S Node.

use crate::accessor::Accessor;
use crate::mesh::Mesh;
use crate::obb::OrientedBoundingBox;
use crate::options::LODSelectionMetric;
use crate::resource::ResourceManager;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Values;
use std::collections::{HashMap, VecDeque};
use std::ops::{Deref, DerefMut, Index};
use std::slice::Iter;
use std::sync::{Arc, Mutex};

/// Helper function to calculate the node page index.
pub fn get_node_page_index(node_index: &usize, nodes_per_page: &usize) -> usize {
    node_index / nodes_per_page
}

/// Helper function to calculate the node index within a node page.
pub fn get_node_index_in_node_page(node_index: &usize, nodes_per_page: &usize) -> usize {
    node_index % nodes_per_page
}

/// I3S Node
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub index: usize,
    pub obb: OrientedBoundingBox,
    #[serde(rename = "parentIndex", default)]
    pub parent_index: Option<usize>,
    #[serde(default)]
    pub children: Vec<usize>,
    #[serde(rename = "lodThreshold", default)]
    pub lod_threshold: Option<f64>,
    #[serde(default)]
    pub mesh: Option<Mesh>,
    #[serde(skip)]
    pub extras: HashMap<String, serde_json::Value>,
    #[serde(skip)]
    pub(crate) cache: Mutex<HashMap<String, serde_json::Value>>,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            obb: self.obb.clone(),
            parent_index: self.parent_index,
            children: self.children.clone(),
            lod_threshold: self.lod_threshold,
            mesh: self.mesh.clone(),
            extras: self.extras.clone(),
            cache: Mutex::new(self.cache.lock().unwrap().clone()), // Clone the inner HashMap for the new Mutex
        }
    }
}

impl Node {
    /// Check if the node is the root node.
    pub fn is_root(&self) -> bool {
        self.parent_index.is_none()
    }

    /// Check if the node is a leaf node.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Get the parent node.
    pub fn get_parent(&self, nodes: &mut NodeArray) -> Option<Arc<Node>> {
        // Check if the node is the root node
        if self.is_root() {
            return None;
        }
        // Get th parent node index
        let parent_index = self.parent_index.as_ref()?;
        // Check if the parent index is valid
        if *parent_index >= nodes.len() {
            return None;
        }
        // Get the parent node from the node array
        let parent_node = nodes.get(parent_index);

        if parent_node.is_none() {
            return None;
        }
        Some(Arc::clone(&parent_node.unwrap()))
    }

    /// Get the children nodes.
    pub fn get_children(&self, nodes: &mut NodeArray) -> Vec<Arc<Node>> {
        self.children
            .iter()
            .filter_map(|&child_index| nodes.get(&child_index))
            .collect()
    }

    /// Get the sibling nodes.
    pub fn get_siblings(&self, nodes: &mut NodeArray) -> Vec<Arc<Node>> {
        self.get_parent(nodes)
            .map(|parent| parent.get_children(nodes))
            .unwrap_or_default()
            .into_iter()
            .filter(|sibling| sibling.index != self.index)
            .collect()
    }
}

/// Node Page
#[derive(Debug, Serialize, Deserialize)]
pub struct NodePage {
    pub nodes: Vec<Node>,
    #[serde(skip)]
    pub index: Option<usize>,
    #[serde(skip)]
    pub extras: HashMap<String, serde_json::Value>,
}

impl Index<usize> for NodePage {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<'a> IntoIterator for &'a NodePage {
    type Item = &'a Node;
    type IntoIter = Iter<'a, Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.iter()
    }
}

impl Deref for NodePage {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl DerefMut for NodePage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.nodes
    }
}

/// Node Page Definition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodePageDefinition {
    #[serde(rename = "nodesPerPage")]
    pub nodes_per_page: usize,
    #[serde(rename = "lodSelectionMetricType")]
    pub lod_selection_metric: LODSelectionMetric,
    #[serde(rename = "rootIndex", default)]
    pub root_index: usize,
}

pub struct NodeArrayIter<'a> {
    node_array: &'a mut NodeArray<'a>,
    current_index: usize,
}

impl<'a> Iterator for NodeArrayIter<'a> {
    type Item = Arc<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        // Try to fetch the next node
        match self.node_array.get(&self.current_index) {
            Some(node) => {
                self.current_index += 1; // Increment the index
                Some(node)
            }
            None => {
                self.current_index = 0;
                None
            } // Stop iteration when `.get` fails
        }
    }
}

impl<'a> NodeArray<'a> {
    pub fn iter(&'a mut self) -> NodeArrayIter<'a> {
        NodeArrayIter {
            node_array: self,
            current_index: 0,
        }
    }
}

/// Node Array
pub struct NodeArray<'a> {
    nodes: HashMap<usize, Arc<Node>>,
    manager: &'a ResourceManager,
}

impl<'a> IntoIterator for &'a NodeArray<'a> {
    type Item = Arc<Node>;
    type IntoIter = std::iter::Map<Values<'a, usize, Arc<Node>>, fn(&Arc<Node>) -> Arc<Node>>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.values().map(Arc::clone)
    }
}

impl<'a> NodeArray<'a> {
    pub fn new(manager: &'a ResourceManager) -> Self {
        Self {
            manager,
            nodes: HashMap::new(),
        }
    }

    pub fn get(&mut self, index: &usize) -> Option<Arc<Node>> {
        // Check if the node is already cached
        if let Some(node) = self.nodes.get(&index) {
            return Some(Arc::clone(node));
        }

        if !self.nodes.contains_key(index) {
            let node = self.manager.get_node(index);

            if node.is_err() {
                return None; // Handle the error as needed
            }
            let node = node.unwrap();
            self.nodes.insert(*index, node);
        }
        let node = Arc::clone(self.nodes.get(&index).unwrap());
        Some(node)
    }

    pub fn traverse<F>(&mut self, mut callback: F)
    where
        F: FnMut(&Arc<Node>, &u8) -> bool,
    {
        let root = self.root();
        let mut stack = VecDeque::new();
        stack.push_back((root, 0));

        while let Some((node, level)) = stack.pop_back() {
            if node.is_none() {
                continue;
            }
            let node = node.unwrap();
            if !callback(&node, &level) {
                return;
            }
            let children = node.get_children(self);
            for child in children {
                let next_level = level + 1;
                stack.push_back((Some(child), next_level));
            }
        }
    }

    pub fn root_index(&self) -> usize {
        let scene_definition = self.manager.scene_definition();
        let node_page_definition = scene_definition.node_pages.as_ref();
        match node_page_definition {
            None => 0,
            Some(node_page_definition) => node_page_definition.root_index,
        }
    }

    pub fn root(&mut self) -> Option<Arc<Node>> {
        let root_index = self.root_index();
        let root_node = self.get(&root_index);
        root_node
    }
}

impl<'a> NodeArray<'a> {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

//! I3S Node.

use crate::mesh::Mesh;
use crate::obb::OrientedBoundingBox;
use crate::options::LODSelectionMetric;
use crate::resource::ResourceManager;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut, Index};
use std::slice::Iter;
// use std::sync::Arc;

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
        }
    }
}

impl Node {
    pub fn is_root(&self) -> bool {
        self.parent_index.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn get_parent<'a>(&'a self, nodes: &'a NodeArray<'a>) -> Option<Node> {
        if self.is_root() {
            return None;
        }

        let parent_index = self.parent_index.as_ref()?;

        let parent_node = nodes.get(parent_index);

        parent_node
    }

    pub fn get_children<'a>(&'a self, nodes: &'a NodeArray<'a>) -> Vec<Node> {
        self.children
            .iter()
            .filter_map(|child_index| nodes.get(child_index))
            .collect()
    }
}

/// Node Page
#[derive(Debug, Serialize, Deserialize)]
pub struct NodePage {
    pub nodes: Vec<Node>,
    #[serde(skip)]
    pub index: Option<usize>,
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

/// Node Array
pub struct NodeArray<'a> {
    nodes: DashMap<usize, Node>,
    manager: &'a ResourceManager,
}

impl<'a> NodeArray<'a> {
    pub fn new(manager: &'a ResourceManager) -> Self {
        Self {
            manager,
            nodes: DashMap::new(),
        }
    }

    pub fn get(&self, index: &usize) -> Option<Node> {
        if !self.nodes.contains_key(index) {
            let node = self.manager.get_node(index);
            if node.is_err() {
                return None;
            }
            let node = node.unwrap();
            self.nodes.insert(*index, node);
        }
        let node = self.nodes.get(index);
        let node = node.unwrap().clone();
        Some(node)
    }

    pub fn traverse<F>(& self, mut callback: F)
    where
        F: FnMut(&Node, &u8) -> bool,
    {
        let root = self.root();
        let mut stack = VecDeque::new();
        if root.is_none() {
            return;
        }
        let root = root.unwrap();
        stack.push_back((root, 0));

        while let Some((node, level)) = stack.pop_front() {
            if !callback(&node, &level) {
                return;
            }
            for child_index in node.children.iter() {
                let child = self.get(child_index);
                if child.is_none() {
                    continue;
                }
                let child = child.unwrap();
                stack.push_back((child, level + 1));
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

    pub fn root(& self) -> Option<Node> {
        let root_index = self.root_index();
        self.get(&root_index)
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

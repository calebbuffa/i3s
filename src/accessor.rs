use std::sync::Arc;

use crate::node::Node;

/// Resource Manager Protocol
pub trait Accessor {
    fn get_node(&self, index: &usize) -> Result<Arc<Node>, String>;
    fn get(&self, uri: &str) -> Result<Vec<u8>, String>;
}

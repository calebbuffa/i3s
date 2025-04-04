pub mod accessor;
pub mod attr;
pub mod crs;
pub mod decode;
pub mod defn;
pub mod geom;
pub mod mesh;
pub mod node;
pub mod obb;
pub mod options;
pub mod resource;
pub mod service;
pub mod slpk;
pub mod uri;
pub mod visual;
use std::sync::Arc;

use resource::{ResourceManager, resource_manager_factory};

use crate::decode::ResourceDecoder;
use crate::defn::SceneDefinition;
use crate::node::{Node, NodeArray};
use crate::options::I3SFormat;

/// SceneLayer
pub struct SceneLayer {
    manager: ResourceManager,
    pub definition: SceneDefinition,
}

impl SceneLayer {
    /// Create a new SceneLayer.
    pub fn new(manager: ResourceManager) -> SceneLayer {
        let definition = manager.scene_definition().to_owned();

        SceneLayer {
            manager,
            definition,
        }
    }

    pub fn create_decoder<'a>(&'a self) -> ResourceDecoder<'a> {
        let decoder = ResourceDecoder::new(&self.manager, &self.definition.store.profile);
        decoder
    }

    pub fn nodes(&self) -> NodeArray {
        NodeArray::new(&self.manager)
    }

    pub fn root(&self) -> Option<Arc<Node>> {
        self.nodes().root()
    }

    /// Create a SceneLayer from a URI.
    pub fn from_uri(uri: &str) -> Result<SceneLayer, String> {
        let fmt = I3SFormat::from_uri(uri)?;
        let manager_factory = resource_manager_factory(fmt);
        let manager = manager_factory(uri);
        Ok(SceneLayer::new(manager))
    }
}

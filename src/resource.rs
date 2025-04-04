//! Resource Managers.

use std::sync::Arc;

use crate::accessor::Accessor;
use crate::defn::SceneDefinition;
use crate::node::Node;
use crate::options::{Compression, I3SFormat};
use crate::service::Service;
use crate::slpk::SceneLayerPackage;
use crate::uri::UriBuilder;

/// Factory for creating Resource Managers.
pub fn resource_manager_factory(fmt: I3SFormat) -> fn(&str) -> ResourceManager {
    match fmt {
        I3SFormat::REST => |uri| ResourceManager::Service(Service::connect(uri).unwrap()),
        I3SFormat::SLPK => {
            |uri| ResourceManager::SceneLayerPackage(SceneLayerPackage::open(uri).unwrap())
        }
    }
}

/// Enum to represent different types of Resource Managers.
pub enum ResourceManager {
    Service(Service),
    SceneLayerPackage(SceneLayerPackage),
}

impl ResourceManager {
    pub fn scene_definition(&self) -> &SceneDefinition {
        match self {
            ResourceManager::Service(service) => &service.scene_definition,
            ResourceManager::SceneLayerPackage(package) => &package.scene_definition,
        }
    }
}

impl Accessor for ResourceManager {
    fn get_node(&self, index: &usize) -> Result<Arc<Node>, String> {
        match self {
            ResourceManager::Service(service) => service.get_node(index),
            ResourceManager::SceneLayerPackage(package) => package.get_node(index),
        }
    }

    fn get(&self, uri: &str) -> Result<Vec<u8>, String> {
        match self {
            ResourceManager::Service(service) => service.get(uri),
            ResourceManager::SceneLayerPackage(package) => package.get(uri),
        }
    }
}

impl UriBuilder for ResourceManager {
    fn create_texture_uri(
        &self,
        resource: &usize,
        name: &str,
        fmt: &str,
        compression: &Compression,
    ) -> Result<String, String> {
        match self {
            ResourceManager::Service(service) => {
                service.create_texture_uri(resource, name, fmt, compression)
            }
            ResourceManager::SceneLayerPackage(package) => {
                package.create_texture_uri(resource, name, fmt, compression)
            }
        }
    }

    fn create_geometry_uri(
        &self,
        resource: &usize,
        compression: &Compression,
    ) -> Result<String, String> {
        match self {
            ResourceManager::Service(service) => service.create_geometry_uri(resource, compression),
            ResourceManager::SceneLayerPackage(package) => {
                package.create_geometry_uri(resource, compression)
            }
        }
    }
}

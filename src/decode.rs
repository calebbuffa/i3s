//! Decoding utility functions.

use crate::accessor::Accessor;
use crate::mesh::{MeshGeometry, MeshMaterial};
use crate::options::{Compression, Profile};
use crate::resource::ResourceManager;
use crate::uri::UriBuilder;
use flate2::read::GzDecoder;
use std::io::Read;
use std::sync::Arc;

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

/// Mesh Pyramid Decoder
pub struct MeshPyramidDecoder<'a> {
    manager: &'a ResourceManager,
    // cache: HashMap<String, Vec<u8>>,
}

impl<'a> MeshPyramidDecoder<'a> {
    /// Create a new MeshPyramidDecoder.
    pub fn new(manager: &'a ResourceManager) -> Self {
        Self { manager }
    }
}

impl<'a> Decoder for MeshPyramidDecoder<'a> {
    fn decode_geometry(
        &self,
        geometry: &mut MeshGeometry,
        compression: &Compression,
    ) -> Result<Arc<Vec<u8>>, String> {
        if geometry.cache.get("data").is_none() {
            let uri = self
                .manager
                .create_geometry_uri(&geometry.resource, compression)?;
            let data = self.manager.get(&uri)?;
            let decompressed = GzDecoder::new(&data[..])
                .bytes()
                .collect::<Result<Vec<u8>, _>>()
                .map_err(|e| format!("Failed to decompress geometry data: {}", e))?;
            geometry
                .cache
                .insert("data".to_string(), Arc::new(decompressed));
        }
        let data = Arc::clone(geometry.cache.get("data").unwrap());
        Ok(data)
    }

    fn decode_material(
        &self,
        material: &mut MeshMaterial,
        compression: &Compression,
    ) -> Result<Arc<Vec<u8>>, String> {
        if material.cache.get("data").is_none() {
            let scene_definition = self.manager.scene_definition();
            let texture_set_definitions = scene_definition
                .texture_set_definitions
                .as_ref()
                .ok_or("Texture set definitions not found in scene definition.")?;
            let texture_def = &texture_set_definitions[material.definition];
            let formats = &texture_def.formats;
            let resource = material.resource;
            let fmt = if *compression == Compression::Compressed {
                &formats[1]
            } else {
                &formats[0]
            };
            let uri = self.manager.create_texture_uri(
                &resource,
                fmt.name.as_str(),
                fmt.format.as_ref(),
                compression,
            )?;
            let data = self.manager.get(&uri)?;
            material.cache.insert("data".to_string(), Arc::new(data));
        }
        let res = Arc::clone(material.cache.get("data").unwrap());
        Ok(res)
    }
}

pub enum ResourceDecoder<'a> {
    MeshPyramid(MeshPyramidDecoder<'a>),
}

impl<'a> ResourceDecoder<'a> {
    /// Create a new ResourceDecoder.
    pub fn new(manager: &'a ResourceManager, profile: &Profile) -> Self {
        match profile {
            Profile::MeshPyramids => ResourceDecoder::MeshPyramid(MeshPyramidDecoder::new(manager)),
            Profile::Points => todo!(),
            Profile::PointClouds => todo!(),
            Profile::Building => todo!(),
        }
    }
}

impl<'a> Decoder for ResourceDecoder<'a> {
    fn decode_geometry(
        &self,
        geometry: &mut MeshGeometry,
        compression: &Compression,
    ) -> Result<Arc<Vec<u8>>, String> {
        match self {
            ResourceDecoder::MeshPyramid(decoder) => decoder.decode_geometry(geometry, compression),
        }
    }

    fn decode_material(
        &self,
        material: &mut MeshMaterial,
        compression: &Compression,
    ) -> Result<Arc<Vec<u8>>, String> {
        match self {
            ResourceDecoder::MeshPyramid(decoder) => decoder.decode_material(material, compression),
        }
    }
}

/// Decoder mapping
pub fn decoder_factory<'a>(
    profile: &Profile,
) -> impl Fn(&'a ResourceManager) -> ResourceDecoder<'a> {
    match profile {
        Profile::MeshPyramids => {
            |manager| ResourceDecoder::MeshPyramid(MeshPyramidDecoder::new(manager))
        }
        Profile::Points => todo!(),
        Profile::PointClouds => todo!(),
        Profile::Building => todo!(),
    }
}

//! Esri Indexed 3D Scene Layers (I3S) specification.
//!
//! This crate is the specification only: typed documents ([`cmn`], [`bld`],
//! [`psl`], [`pcsl`]), package hash canonicalization,
//! node-page arithmetic, and the binary layouts of uncompressed geometry and
//! attribute buffers. It performs no I/O,
//! owns no threads, and links no compression or mesh-compression codec.
//!
#![warn(missing_docs)]

mod attributes;
mod binary;
mod document;
mod error;
mod generated;
mod geometry;
mod identity;
mod pages;
mod slpk;

pub use attributes::{AttributeBuffer, AttributeValues, PropertyValue};
pub use attributes::{decode_attribute, encode_attribute};
pub use document::{NodePageDocument, SceneLayerDocument};
pub use error::Error;
pub use generated::{bld, cmn, pcsl, psl};
pub use geometry::{Geometry, GeometryCounts, GeometryDescriptor, GeometryValues};
pub use geometry::{decode_geometry, decode_schema_geometry, encode_geometry_checked};
pub use identity::{NodeId, SublayerId};
pub use pages::{node_page_count, pack_node_pages, page_id_of, page_offset_of};
pub use slpk::slpk_hash_path;

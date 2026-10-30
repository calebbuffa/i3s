//! Esri Indexed 3D Scene Layers (I3S) specification.
//!
//! This crate is the specification only: typed documents ([`cmn`], [`bld`],
//! [`psl`], [`pcsl`]), resource naming ([`Source`], [`ResourceLocation`]),
//! node-page arithmetic, and the binary layouts of uncompressed geometry and
//! attribute buffers. It performs no I/O,
//! owns no threads, and links no compression or mesh-compression codec.
//!
//! Reading, writing, hierarchy traversal and navigation are provided by the
//! `hiera` crate's `i3s` feature, which schedules transport and decoding
//! under application control.
//!
//! ```
//! use i3s::{ResourceLocation, Source};
//!
//! let source = Source::rest("https://example.com/SceneServer", 0);
//! let uri = ResourceLocation::NodePage(3).resolve(&source);
//! assert!(uri.ends_with("nodepages/3"));
//! ```

#![warn(missing_docs)]

mod attributes;
mod binary;
mod descriptor;
mod document;
mod error;
mod generated;
mod geometry;
mod identity;
mod pages;
mod resource;
mod urls;

pub use attributes::{AttributeBuffer, AttributeDecodeError, AttributeValues, PropertyValue};
pub use attributes::{decode_attribute, encode_attribute};
pub use descriptor::GeometryDescriptor;
pub use document::{NodePageDocument, SceneLayerDocument};
pub use error::Error;
pub use generated::{bld, cmn, pcsl, psl};
pub use geometry::{Geometry, GeometryCounts, GeometryError, GeometryValues};
pub use geometry::{decode_geometry, decode_schema_geometry, encode_geometry_checked};
pub use identity::{NodeId, SublayerId};
pub use pages::{node_page_count, pack_node_pages};
pub use resource::{ResourceLocation, ResourcePath};
pub use urls::{Source, SourceKind, canonical_path, page_id_of, page_offset_of};

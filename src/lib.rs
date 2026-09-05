//! Reading and writing Esri Indexed 3D Scene Layers (I3S).
//!
//! The types in [`generated`] are produced from the I3S specification's own
//! markdown by the `i3s-schema` converter and `schemagen`, so they track the
//! specification rather than a hand transcription of it. Around them this
//! crate adds the parts a client needs but the schema cannot express:
//!
//! * [`Source`] — resource naming for REST services, `.slpk` packages and
//!   exploded packages, which differ in more than just a prefix.
//! * [`decode_geometry`] / [`decode_legacy_geometry`] — the 1.7+ fixed-order
//!   geometry buffer and the 1.6 `defaultGeometrySchema` layout.
//! * [`decode_i3s_attributes`] — binary attribute buffers.
//! * [`SceneLayerLoader`] — demand-driven traversal via the `kiba::Loader`
//!   protocol.

mod attributes;
mod binary;
mod generated;
mod geometry;
mod loader;
pub use generated::{bld, cmn, pcsl, psl};
mod error;
mod select;
mod urls;

pub use attributes::{
    AttributeBuffer, AttributeDecodeError, AttributeValues, PropertyValue, decode_attribute,
    decode_i3s_attributes,
};
pub use error::Error;
pub use geometry::{
    Geometry, GeometryCounts, GeometryError, GeometryValues, decode_geometry,
    decode_legacy_geometry, encode_geometry,
};
pub use loader::{ContentKind, ContentRef, LoadedContent, NodeRef, SceneLayerLoader};
pub use select::{
    GeometryBufferChoice, GeometryEncodingPreference, select_buffer_in_definition,
    select_geometry_buffer,
};
pub use urls::{Source, SourceKind, canonical_path, page_id_of, page_offset_of};

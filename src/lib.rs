//! Reading Esri Indexed 3D Scene Layers (I3S).
//!
//! The types in [`generated`] are produced from the I3S specification's own
//! markdown by the `i3s-schema` converter and `schemagen`, so they track the
//! specification rather than a hand transcription of it. Around them this
//! crate adds the parts a client needs but the schema cannot express:
//!
//! * [`Source`] - resource naming for REST services, `.slpk` packages and
//!   exploded packages, which differ in more than just a prefix;
//! * wire codecs for the I3S geometry and attribute binary layouts;
//! * [`SceneLayerLoader`] - demand-driven hierarchy discovery;
//! * [`SceneLayerReader`] - a typed I3S reader that decorates a loader,
//!   retaining all [`hiera::Loader`] functionality while adding direct typed
//!   document and resource reads;
//! * [`SceneLayerNavigator`] - retainable, path-aware cursors over a layer;
//! * [`SceneLayerStore`] - caller-owned resource transport.
//!
//! The crate deliberately stops at the specification's boundary. Package
//! compilation, merge strategy, edit policy, and physical `.slpk`/`.eslpk`
//! emission are application decisions, so they are not provided here.
//!
//! # Traversal
//!
//! Transport and thread placement belong to the caller. A loader is given a
//! fetch callback, and traversal is driven by the application:
//!
//! ```no_run
//! use i3s::{SceneLayerLoader, SceneLayerNavigator, SceneLayerReader, Source};
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), i3s::Error> {
//! let source = Source::rest("https://example.com/SceneServer", 0);
//! let loader = SceneLayerLoader::open(source, |_: hiera::FetchRequest| async move {
//!     // The application owns transport: HTTP, filesystem, archive, or cache.
//!     Ok::<_, hiera::FetchError>(hiera::FetchResponse {
//!         bytes: hiera::Bytes::new(),
//!         content_type: None,
//!     })
//! });
//!
//! let navigator = SceneLayerNavigator::new(SceneLayerReader::new(Arc::new(loader)));
//! let root = navigator.root().await?;
//! for child in root.children().await? {
//!     println!("{}", child.id());
//! }
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]

mod attributes;
mod binary;
mod codec;
mod generated;
mod geometry;
mod identity;
mod loader;
mod navigation;
mod reader;
mod resource;
mod store;
pub use generated::{bld, cmn, pcsl, psl};
mod error;
mod select;
mod urls;

pub use attributes::{AttributeBuffer, AttributeDecodeError, AttributeValues, PropertyValue};
pub use codec::{
    GeometryAdapter, GeometryDescriptor, LegacyGeometryCodec, ResourceCodec,
    StandardAttributeCodec, UncompressedGeometryCodec, scalar_count,
};
pub use error::Error;
pub use geometry::{Geometry, GeometryCounts, GeometryError, GeometryValues};
pub use identity::{AttributeKey, NodeId, NodePageId, ResourceId, SublayerId};
pub use loader::{
    ContentKind, ContentRef, LoadedContent, NodeBounds, NodeInfo, NodeRef, SceneLayerLoader,
};
pub use navigation::{
    SceneLayerCursor, SceneLayerCursorState, SceneLayerExpansion, SceneLayerNavigator,
};
pub use reader::{NodePageDocument, SceneLayerDocument, SceneLayerReader};
pub use resource::{ResourceLocation, ResourcePath};
pub use select::{
    GeometryBufferChoice, GeometryEncodingPreference, select_buffer_in_definition,
    select_geometry_buffer,
};
pub use store::{FetchStore, SceneLayerStore};
pub use urls::{Source, SourceKind, canonical_path, page_id_of, page_offset_of};

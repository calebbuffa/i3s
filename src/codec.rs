//! Extensible I3S binary resource codecs.

use std::error::Error as StdError;

use crate::{
    AttributeBuffer, AttributeDecodeError, Geometry, GeometryCounts, GeometryError, GeometryValues,
    cmn::{AttributeStorageInfo, DefaultGeometrySchema, GeometryBuffer},
};

/// A bidirectional codec for an I3S binary resource layout.
pub trait ResourceCodec {
    /// Descriptor from the layer or node metadata.
    type Descriptor;
    /// Editable decoded representation.
    type Value;
    /// Codec failure.
    type Error: StdError + Send + Sync + 'static;

    /// Decodes bytes according to a source descriptor.
    fn decode(
        &self,
        bytes: &[u8],
        descriptor: &Self::Descriptor,
    ) -> Result<Self::Value, Self::Error>;

    /// Validates and encodes an editable value according to a descriptor.
    fn encode(
        &self,
        value: &Self::Value,
        descriptor: &Self::Descriptor,
    ) -> Result<Vec<u8>, Self::Error>;
}

/// Descriptor required to decode a 1.7+ uncompressed geometry buffer.
#[derive(Clone)]
pub struct GeometryDescriptor {
    /// The selected buffer definition.
    pub buffer: GeometryBuffer,
    /// Counts declared by the node page.
    pub counts: GeometryCounts,
}

/// Built-in codec for I3S 1.7+ uncompressed geometry buffers.
#[derive(Debug, Clone, Copy, Default)]
pub struct UncompressedGeometryCodec;

impl ResourceCodec for UncompressedGeometryCodec {
    type Descriptor = GeometryDescriptor;
    type Value = Geometry;
    type Error = GeometryError;

    fn decode(
        &self,
        bytes: &[u8],
        descriptor: &Self::Descriptor,
    ) -> Result<Self::Value, Self::Error> {
        crate::geometry::decode_geometry(&descriptor.buffer, bytes, descriptor.counts)
    }

    fn encode(
        &self,
        value: &Self::Value,
        descriptor: &Self::Descriptor,
    ) -> Result<Vec<u8>, Self::Error> {
        crate::geometry::encode_geometry_checked(&descriptor.buffer, value, descriptor.counts)
    }
}

/// Built-in decoder for I3S 1.6 legacy geometry buffers.
#[derive(Debug, Clone, Copy, Default)]
pub struct LegacyGeometryCodec;

impl LegacyGeometryCodec {
    /// Decodes a legacy buffer described by `defaultGeometrySchema`.
    pub fn decode(
        &self,
        bytes: &[u8],
        schema: &DefaultGeometrySchema,
    ) -> Result<Geometry, GeometryError> {
        crate::geometry::decode_legacy_geometry(schema, bytes)
    }
}

/// Built-in codec for standard I3S attribute buffers.
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardAttributeCodec;

impl StandardAttributeCodec {
    /// Decodes one node attribute buffer.
    pub fn decode(
        &self,
        bytes: &[u8],
        descriptor: &AttributeStorageInfo,
    ) -> Result<AttributeBuffer, AttributeDecodeError> {
        crate::attributes::decode_attribute(bytes, descriptor)
    }

    /// Encodes one validated node attribute buffer.
    pub fn encode(
        &self,
        value: &AttributeBuffer,
        descriptor: &AttributeStorageInfo,
    ) -> Result<Vec<u8>, AttributeDecodeError> {
        crate::attributes::encode_attribute(value, descriptor)
    }
}

/// Converts a decoded geometry attribute to an application-owned type.
///
/// Applications can implement this around `geo`, a mesh engine, or a custom
/// streaming representation without making `i3s` depend on that library.
pub trait GeometryAdapter {
    /// Application representation.
    type Geometry;
    /// Adapter failure.
    type Error: StdError + Send + Sync + 'static;

    /// Converts an I3S wire representation to the application representation.
    fn import(&self, geometry: Geometry) -> Result<Self::Geometry, Self::Error>;

    /// Converts an application representation back to I3S wire values.
    fn export(&self, geometry: Self::Geometry) -> Result<Geometry, Self::Error>;
}

/// Returns the scalar count of a decoded geometry value.
pub fn scalar_count(values: &GeometryValues) -> usize {
    values.len()
}

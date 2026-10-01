//! Decoding and encoding of I3S geometry buffers.
//!
//! I3S has two geometry layouts, and a compliant client must handle both:
//!
//! * **1.7+ `geometryBuffer`** - the layer's `geometryDefinitions` describe
//!   the binary layout declaratively. Attributes appear in a **fixed order**
//!   that the specification pins down so that parsing needs no lookup:
//!
//!   ```text
//!   position  normal  uv0  uv1  color  uvRegion  featureId  faceRange
//!   ```
//!
//!   > **Important**: The order of the vertex attributes in the buffer is
//!   > **fixed** to simplify binary parsing.
//!   >
//!   > -- `geometryBuffer.cmn.md`
//!
//!   Each block is contiguous and its size is exactly
//!   `component * sizeof(type) * count`, where `count` is the vertex count for
//!   per-vertex bindings and the feature count for per-feature bindings.
//!
//! * **1.6 legacy `store.defaultGeometrySchema`** - the layout is described by
//!   a `header` list, an `ordering` list for vertex attributes and a
//!   `featureAttributeOrder` list for per-feature attributes. The vertex and
//!   feature counts are read from the header rather than supplied by the
//!   caller.
//!
//! Both layouts are densely packed little-endian with **no alignment padding**
//! (see [`crate::binary`]).
//!
//! # Positions are OBB-relative
//!
//! > Vertex positions relative to oriented-bounding-box center.
//! >
//! > -- `geometryBuffer.cmn.md`
//!
//! [`decode_geometry`] returns positions exactly as stored; add the node's
//! oriented bounding box centre to obtain absolute coordinates.
//!
//! # Draco
//!
//! A buffer carrying `compressedAttributes` is Draco-encoded and cannot be
//! decoded here - Draco is a separate codec and pulling it in would contradict
//! this crate's dependency-free goal. Such buffers are reported as
//! [`GeometryError::Compressed`] so callers can route them to a Draco decoder;
//! [`select_geometry_buffer`](crate::select_geometry_buffer) can be told to
//! avoid them.

use crate::binary::{BufferReader, UnexpectedEndOfData, write_le_slice};
use crate::cmn::{
    DefaultGeometrySchema, DefaultGeometrySchemaTopology, GeometryAttribute,
    GeometryAttributeValueType, GeometryBuffer, GeometryColorType, GeometryFaceRangeType,
    GeometryFeatureIdType, GeometryNormalType, GeometryPositionType, GeometryUvRegionType,
    GeometryUvType, HeaderAttributeType,
};

/// A failure while decoding or encoding a geometry buffer.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum GeometryError {
    /// The buffer ended before the declared layout was satisfied.
    #[error("geometry buffer ended early while reading `{attribute}`")]
    UnexpectedEnd {
        /// The attribute being read when the buffer ran out.
        attribute: &'static str,
    },

    /// The buffer is Draco-compressed and must be handed to a Draco decoder.
    #[error("geometry buffer uses `compressedAttributes` (Draco) encoding")]
    Compressed,

    /// Bytes remained after every declared attribute had been consumed. The
    /// declared layout does not match the payload, so the decode is unsound.
    #[error("geometry buffer has {trailing} trailing byte(s) after the declared layout")]
    TrailingBytes {
        /// Number of bytes left over.
        trailing: usize,
    },

    /// The layout declared a topology this decoder does not implement.
    #[error("unsupported geometry topology `{topology}`")]
    UnsupportedTopology {
        /// The declared topology.
        topology: &'static str,
    },

    /// A legacy schema named an attribute the specification does not define.
    #[error("unknown legacy geometry attribute `{name}`")]
    UnknownAttribute {
        /// The offending name from `ordering` or `featureAttributeOrder`.
        name: String,
    },

    /// A legacy header lacked `vertexCount`, so no block size can be derived.
    #[error("legacy geometry header has no `vertexCount` property")]
    MissingVertexCount,

    /// An edited geometry did not match its declared buffer layout.
    #[error("geometry attribute `{attribute}` does not match the declared layout")]
    LayoutMismatch {
        /// The declared attribute that failed validation.
        attribute: &'static str,
    },
}

impl From<UnexpectedEndOfData> for GeometryError {
    fn from(_: UnexpectedEndOfData) -> Self {
        Self::UnexpectedEnd {
            attribute: "unknown",
        }
    }
}

/// The values of one decoded geometry attribute, widened to a common type per
/// storage class.
///
/// Widening is deliberate and cheap: I3S stores each attribute at exactly one
/// width, so this enum has one variant per class rather than per declared
/// type, and callers do not need to match on eight integer widths.
#[derive(Debug, Clone, PartialEq)]
pub enum GeometryValues {
    /// `Float32` data (positions, normals, UVs).
    F32(Vec<f32>),
    /// `Float64` data.
    F64(Vec<f64>),
    /// `UInt8` data (colours).
    U8(Vec<u8>),
    /// `UInt16` data (UV regions, narrow feature ids).
    U16(Vec<u16>),
    /// `UInt32` data (face ranges, feature ids).
    U32(Vec<u32>),
    /// `UInt64` data (wide feature ids).
    U64(Vec<u64>),
    /// Signed 16-bit data.
    I16(Vec<i16>),
    /// Signed 32-bit data.
    I32(Vec<i32>),
    /// Signed 64-bit data.
    I64(Vec<i64>),
}

impl GeometryValues {
    /// Number of scalar values (not elements) held.
    pub fn len(&self) -> usize {
        match self {
            Self::F32(v) => v.len(),
            Self::F64(v) => v.len(),
            Self::U8(v) => v.len(),
            Self::U16(v) => v.len(),
            Self::U32(v) => v.len(),
            Self::U64(v) => v.len(),
            Self::I16(v) => v.len(),
            Self::I32(v) => v.len(),
            Self::I64(v) => v.len(),
        }
    }

    /// Whether no values are held.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The `Float32` values, if this is [`GeometryValues::F32`].
    ///
    /// Convenience for the common case of positions, normals and UVs.
    pub fn as_f32(&self) -> Option<&[f32]> {
        match self {
            Self::F32(v) => Some(v),
            _ => None,
        }
    }

    fn write_le(&self, out: &mut Vec<u8>) {
        match self {
            Self::F32(v) => write_le_slice(out, v),
            Self::F64(v) => write_le_slice(out, v),
            Self::U8(v) => write_le_slice(out, v),
            Self::U16(v) => write_le_slice(out, v),
            Self::U32(v) => write_le_slice(out, v),
            Self::U64(v) => write_le_slice(out, v),
            Self::I16(v) => write_le_slice(out, v),
            Self::I32(v) => write_le_slice(out, v),
            Self::I64(v) => write_le_slice(out, v),
        }
    }
}

/// A fully decoded geometry buffer.
///
/// Every field is `None` when the corresponding attribute was absent from the
/// declared layout - I3S omits absent attributes entirely rather than storing
/// a zeroed block.
#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct Geometry {
    /// Vertex positions, relative to the node's oriented bounding box centre.
    pub position: Option<GeometryValues>,
    /// Vertex or face normals.
    pub normal: Option<GeometryValues>,
    /// First texture coordinate set.
    pub uv0: Option<GeometryValues>,
    /// Vertex colours, normally `UInt8` RGBA.
    pub color: Option<GeometryValues>,
    /// Atlas sub-image regions, `[umin, vmin, umax, vmax]` per vertex.
    pub uv_region: Option<GeometryValues>,
    /// Per-feature identifiers.
    pub feature_id: Option<GeometryValues>,
    /// Per-feature **inclusive** first/last triangle indices.
    ///
    /// > `faceRange` is an inclusive range of faces of the geometry that
    /// > belongs to a specific feature.
    /// >
    /// > -- `geometryFaceRange.cmn.md`
    pub face_range: Option<GeometryValues>,
    /// Face indices, present only for legacy `Indexed` topology.
    pub faces: Option<GeometryValues>,
    /// Number of vertices the layout described.
    pub vertex_count: usize,
    /// Number of features the layout described.
    pub feature_count: usize,
}

/// How many vertices and features a 1.7+ buffer contains.
///
/// The 1.7+ `geometryBuffer` layout is purely declarative - it says how wide
/// each element is but not how many there are. Those counts live in the node
/// page (`mesh.geometry.vertexCount` / `featureCount`), so the caller supplies
/// them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GeometryCounts {
    /// Number of vertices in the buffer.
    pub vertex_count: usize,
    /// Number of features in the buffer.
    pub feature_count: usize,
}

impl GeometryCounts {
    /// Counts for a buffer with `vertex_count` vertices and no features.
    pub fn vertices(vertex_count: usize) -> Self {
        Self {
            vertex_count,
            feature_count: 0,
        }
    }
}

/// Per-attribute layout: how many components, at what width, bound to what.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Layout {
    component: usize,
    value_type: ValueType,
    per_feature: bool,
}

/// Every scalar width I3S geometry can store.
///
/// The schema gives each attribute its own type enum (`GeometryPositionType`,
/// `GeometryColorType`, and so on), each listing only the widths that attribute
/// permits. None of them covers the full set, so decoding needs one internal
/// enum that does - crucially including `UInt32` and `UInt64`, which
/// `faceRange` and `featureId` use but `GeometryAttributeValueType` omits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
}

impl From<GeometryAttributeValueType> for ValueType {
    fn from(value: GeometryAttributeValueType) -> Self {
        match value {
            GeometryAttributeValueType::UInt8 => Self::UInt8,
            GeometryAttributeValueType::UInt16 => Self::UInt16,
            GeometryAttributeValueType::Int16 => Self::Int16,
            GeometryAttributeValueType::Int32 => Self::Int32,
            GeometryAttributeValueType::Int64 => Self::Int64,
            GeometryAttributeValueType::Float32 => Self::Float32,
            GeometryAttributeValueType::Float64 => Self::Float64,
        }
    }
}

impl From<HeaderAttributeType> for ValueType {
    fn from(value: HeaderAttributeType) -> Self {
        match value {
            HeaderAttributeType::UInt8 => Self::UInt8,
            HeaderAttributeType::UInt16 => Self::UInt16,
            HeaderAttributeType::UInt32 => Self::UInt32,
            HeaderAttributeType::UInt64 => Self::UInt64,
            HeaderAttributeType::Int16 => Self::Int16,
            HeaderAttributeType::Int32 => Self::Int32,
            HeaderAttributeType::Int64 => Self::Int64,
            HeaderAttributeType::Float32 => Self::Float32,
            HeaderAttributeType::Float64 => Self::Float64,
        }
    }
}

/// Maps each attribute's own narrow type enum onto [`ValueType`].
///
/// One `impl` per generated enum keeps the mapping exhaustive: if the schema
/// gains a variant, the corresponding `match` stops compiling rather than
/// silently falling back to a wrong width.
macro_rules! impl_value_type {
    ($($enum:ty { $($variant:ident => $width:ident),* $(,)? })*) => {
        $(impl From<$enum> for ValueType {
            fn from(value: $enum) -> Self {
                match value {
                    $(<$enum>::$variant => Self::$width),*
                }
            }
        })*
    };
}

impl_value_type! {
    GeometryPositionType { Float32 => Float32 }
    GeometryNormalType { Float32 => Float32 }
    GeometryUvType { Float32 => Float32 }
    GeometryColorType { UInt8 => UInt8 }
    GeometryUvRegionType { UInt16 => UInt16 }
    GeometryFaceRangeType { UInt32 => UInt32 }
    GeometryFeatureIdType {
        UInt16 => UInt16,
        UInt32 => UInt32,
        UInt64 => UInt64,
    }
}

impl Layout {
    fn count(self, counts: GeometryCounts) -> usize {
        let elements = if self.per_feature {
            counts.feature_count
        } else {
            counts.vertex_count
        };
        elements * self.component
    }
}

fn read_values(
    reader: &mut BufferReader<'_>,
    layout: Layout,
    counts: GeometryCounts,
    attribute: &'static str,
) -> Result<GeometryValues, GeometryError> {
    read_n(reader, layout.value_type, layout.count(counts), attribute)
}

fn read_n(
    reader: &mut BufferReader<'_>,
    value_type: ValueType,
    n: usize,
    attribute: &'static str,
) -> Result<GeometryValues, GeometryError> {
    let map = |_: UnexpectedEndOfData| GeometryError::UnexpectedEnd { attribute };
    Ok(match value_type {
        ValueType::Float32 => GeometryValues::F32(reader.read_le_vec::<f32>(n).map_err(map)?),
        ValueType::Float64 => GeometryValues::F64(reader.read_le_vec::<f64>(n).map_err(map)?),
        ValueType::UInt8 => GeometryValues::U8(reader.read_le_vec::<u8>(n).map_err(map)?),
        ValueType::UInt16 => GeometryValues::U16(reader.read_le_vec::<u16>(n).map_err(map)?),
        ValueType::UInt32 => GeometryValues::U32(reader.read_le_vec::<u32>(n).map_err(map)?),
        ValueType::UInt64 => GeometryValues::U64(reader.read_le_vec::<u64>(n).map_err(map)?),
        ValueType::Int16 => GeometryValues::I16(reader.read_le_vec::<i16>(n).map_err(map)?),
        ValueType::Int32 => GeometryValues::I32(reader.read_le_vec::<i32>(n).map_err(map)?),
        ValueType::Int64 => GeometryValues::I64(reader.read_le_vec::<i64>(n).map_err(map)?),
    })
}

/// Decodes a 1.7+ `geometryBuffer` payload.
///
/// `definition` is the chosen `GeometryBuffer` from
/// `layer.geometryDefinitions[..].geometryBuffers[..]`; `counts` come from the
/// node page's `mesh.geometry`.
///
/// Attributes are read in the specification's fixed order. Any leading header
/// described by `definition.offset` is skipped.
///
/// # Errors
///
/// Returns [`GeometryError::Compressed`] for a Draco buffer,
/// [`GeometryError::UnexpectedEnd`] if the payload is shorter than the
/// declared layout, and [`GeometryError::TrailingBytes`] if it is longer -
/// the latter signals that the layout and payload disagree, which would make
/// any decoded values untrustworthy.
///
/// ```
/// # use i3s::{GeometryCounts, GeometryDescriptor, ResourceCodec, UncompressedGeometryCodec};
/// # use i3s::cmn::{GeometryBuffer, GeometryPosition, GeometryPositionType};
/// let definition = GeometryBuffer {
///     position: Some(GeometryPosition {
///         component: 3,
///         r#type: GeometryPositionType::Float32,
///         ..Default::default()
///     }),
///     ..Default::default()
/// };
/// let mut bytes = Vec::new();
/// for v in [1.0f32, 2.0, 3.0] {
///     bytes.extend_from_slice(&v.to_le_bytes());
/// }
/// let geometry = UncompressedGeometryCodec.decode(
///     &bytes,
///     &GeometryDescriptor { buffer: definition, counts: GeometryCounts::vertices(1) },
/// ).unwrap();
/// assert_eq!(geometry.position.unwrap().as_f32(), Some(&[1.0, 2.0, 3.0][..]));
/// ```
pub fn decode_geometry(
    definition: &GeometryBuffer,
    bytes: &[u8],
    counts: GeometryCounts,
) -> Result<Geometry, GeometryError> {
    if definition.compressed_attributes.is_some() {
        return Err(GeometryError::Compressed);
    }

    let mut reader = BufferReader::new(bytes);
    // `offset` skips a legacy header that this layout does not describe.
    reader
        .seek(definition.offset.unwrap_or(0) as usize)
        .map_err(|_| GeometryError::UnexpectedEnd {
            attribute: "offset",
        })?;

    let mut geometry = Geometry {
        vertex_count: counts.vertex_count,
        feature_count: counts.feature_count,
        ..Default::default()
    };

    // The specification fixes this order; do not reorder these reads.
    macro_rules! read_attr {
        ($field:ident, $name:literal, $per_feature:expr) => {
            if let Some(attr) = &definition.$field {
                let layout = Layout {
                    component: attr.component as usize,
                    value_type: ValueType::from(attr.r#type),
                    per_feature: $per_feature,
                };
                geometry.$field = Some(read_values(&mut reader, layout, counts, $name)?);
            }
        };
    }

    read_attr!(position, "position", false);
    read_attr!(normal, "normal", false);
    read_attr!(uv0, "uv0", false);
    read_attr!(color, "color", false);
    read_attr!(uv_region, "uvRegion", false);
    read_attr!(feature_id, "featureId", true);
    read_attr!(face_range, "faceRange", true);

    let trailing = reader.remaining();
    if trailing != 0 {
        return Err(GeometryError::TrailingBytes { trailing });
    }
    Ok(geometry)
}

/// Encodes a [`Geometry`] into a 1.7+ `geometryBuffer` payload.
///
/// The inverse of [`decode_geometry`]: attributes are written in the
/// specification's fixed order, densely packed and little-endian, preceded by
/// `definition.offset` zero bytes so that the result round-trips.
///
/// ```
/// # use i3s::{GeometryCounts, GeometryDescriptor, ResourceCodec, UncompressedGeometryCodec};
/// # use i3s::cmn::{GeometryBuffer, GeometryPosition, GeometryPositionType};
/// let definition = GeometryBuffer {
///     position: Some(GeometryPosition {
///         component: 3,
///         r#type: GeometryPositionType::Float32,
///         ..Default::default()
///     }),
///     ..Default::default()
/// };
/// let mut bytes = Vec::new();
/// for v in [1.0f32, 2.0, 3.0] {
///     bytes.extend_from_slice(&v.to_le_bytes());
/// }
/// let counts = GeometryCounts::vertices(1);
/// let descriptor = GeometryDescriptor { buffer: definition, counts };
/// let geometry = UncompressedGeometryCodec.decode(&bytes, &descriptor).unwrap();
/// assert_eq!(UncompressedGeometryCodec.encode(&geometry, &descriptor).unwrap(), bytes);
/// ```
pub fn encode_geometry(
    definition: &GeometryBuffer,
    geometry: &Geometry,
) -> Result<Vec<u8>, GeometryError> {
    if definition.compressed_attributes.is_some() {
        return Err(GeometryError::Compressed);
    }

    let mut out = vec![0u8; definition.offset.unwrap_or(0) as usize];
    for values in [
        geometry.position.as_ref(),
        geometry.normal.as_ref(),
        geometry.uv0.as_ref(),
        geometry.color.as_ref(),
        geometry.uv_region.as_ref(),
        geometry.feature_id.as_ref(),
        geometry.face_range.as_ref(),
    ]
    .into_iter()
    .flatten()
    {
        values.write_le(&mut out);
    }
    Ok(out)
}

/// Validates and encodes a 1.7+ geometry buffer for use by a writer.
pub(crate) fn encode_geometry_checked(
    definition: &GeometryBuffer,
    geometry: &Geometry,
    counts: GeometryCounts,
) -> Result<Vec<u8>, GeometryError> {
    if geometry.vertex_count != counts.vertex_count
        || geometry.feature_count != counts.feature_count
    {
        return Err(GeometryError::LayoutMismatch {
            attribute: "counts",
        });
    }
    macro_rules! validate {
        ($field:ident, $name:literal, $per_feature:expr) => {
            match (&definition.$field, &geometry.$field) {
                (None, None) => {}
                (Some(declared), Some(values)) => {
                    let layout = Layout {
                        component: declared.component as usize,
                        value_type: ValueType::from(declared.r#type),
                        per_feature: $per_feature,
                    };
                    if !matches_value_type(values, layout.value_type)
                        || values.len() != layout.count(counts)
                    {
                        return Err(GeometryError::LayoutMismatch { attribute: $name });
                    }
                }
                _ => return Err(GeometryError::LayoutMismatch { attribute: $name }),
            }
        };
    }
    validate!(position, "position", false);
    validate!(normal, "normal", false);
    validate!(uv0, "uv0", false);
    validate!(color, "color", false);
    validate!(uv_region, "uvRegion", false);
    validate!(feature_id, "featureId", true);
    validate!(face_range, "faceRange", true);
    encode_geometry(definition, geometry)
}

fn matches_value_type(values: &GeometryValues, value_type: ValueType) -> bool {
    matches!(
        (values, value_type),
        (GeometryValues::F32(_), ValueType::Float32)
            | (GeometryValues::F64(_), ValueType::Float64)
            | (GeometryValues::U8(_), ValueType::UInt8)
            | (GeometryValues::U16(_), ValueType::UInt16)
            | (GeometryValues::U32(_), ValueType::UInt32)
            | (GeometryValues::U64(_), ValueType::UInt64)
            | (GeometryValues::I16(_), ValueType::Int16)
            | (GeometryValues::I32(_), ValueType::Int32)
            | (GeometryValues::I64(_), ValueType::Int64)
    )
}

/// Decodes a 1.6 legacy geometry buffer described by
/// `store.defaultGeometrySchema`.
///
/// Unlike the 1.7+ layout, the counts are *in* the payload: the `header` list
/// names the properties (`vertexCount`, `featureCount`) that precede the
/// vertex data, and this function reads them rather than taking them as
/// arguments. Vertex attributes then follow in `ordering` order, and
/// per-feature attributes in `featureAttributeOrder` order.
///
/// `Indexed` topology places the face indices immediately after the header and
/// before the vertex attributes, as required by the schema:
///
/// > When 'Indexed', the indices must also be declared in the geometry schema
/// > ('faces') and precede the vertexAttribute data.
/// >
/// > -- `defaultGeometrySchema.cmn.md`
pub fn decode_legacy_geometry(
    schema: &DefaultGeometrySchema,
    bytes: &[u8],
) -> Result<Geometry, GeometryError> {
    let mut reader = BufferReader::new(bytes);

    // The header is a flat list of scalars; `vertexCount` and `featureCount`
    // are the two the layout depends on.
    let mut vertex_count: Option<usize> = None;
    let mut feature_count: usize = 0;
    for entry in &schema.header {
        let value = read_n(&mut reader, ValueType::from(entry.r#type), 1, "header")?;
        let scalar = match &value {
            GeometryValues::U8(v) => v.first().map(|&x| x as usize),
            GeometryValues::U16(v) => v.first().map(|&x| x as usize),
            GeometryValues::U32(v) => v.first().map(|&x| x as usize),
            GeometryValues::U64(v) => v.first().map(|&x| x as usize),
            GeometryValues::I16(v) => v.first().map(|&x| x.max(0) as usize),
            GeometryValues::I32(v) => v.first().map(|&x| x.max(0) as usize),
            GeometryValues::I64(v) => v.first().map(|&x| x.max(0) as usize),
            _ => None,
        };
        match entry.property.as_ref() {
            "vertexCount" => vertex_count = scalar,
            "featureCount" => feature_count = scalar.unwrap_or(0),
            _ => {}
        }
    }
    let vertex_count = vertex_count.ok_or(GeometryError::MissingVertexCount)?;

    let mut geometry = Geometry {
        vertex_count,
        feature_count,
        ..Default::default()
    };

    let indexed = schema.topology == DefaultGeometrySchemaTopology::Indexed;
    if indexed {
        // `faces` is itself a `vertexAttribute`; its `position` member holds
        // the index declaration.
        let faces = schema
            .faces
            .as_ref()
            .ok_or(GeometryError::UnsupportedTopology {
                topology: "Indexed without a `faces` declaration",
            })?;
        geometry.faces = Some(read_legacy_attribute(
            &mut reader,
            &faces.position,
            vertex_count,
            "faces",
        )?);
    }

    for name in &schema.ordering {
        let attribute = legacy_vertex_attribute(schema, name)?;
        let values =
            read_legacy_attribute(&mut reader, attribute, vertex_count, "vertexAttribute")?;
        store_vertex_attribute(&mut geometry, name, values)?;
    }

    for name in &schema.feature_attribute_order {
        let values = read_legacy_feature_attribute(&mut reader, schema, name, feature_count)?;
        store_feature_attribute(&mut geometry, name, values)?;
    }

    let trailing = reader.remaining();
    if trailing != 0 {
        return Err(GeometryError::TrailingBytes { trailing });
    }
    Ok(geometry)
}

fn read_legacy_attribute(
    reader: &mut BufferReader<'_>,
    attribute: &GeometryAttribute,
    elements: usize,
    label: &'static str,
) -> Result<GeometryValues, GeometryError> {
    // A `byteOffset` on a legacy attribute is absolute within the buffer.
    if let Some(offset) = attribute.byte_offset {
        reader
            .seek(offset as usize)
            .map_err(|_| GeometryError::UnexpectedEnd { attribute: label })?;
    }
    let layout = Layout {
        component: attribute.values_per_element as usize,
        value_type: ValueType::from(attribute.value_type),
        per_feature: false,
    };
    read_values(reader, layout, GeometryCounts::vertices(elements), label)
}

fn legacy_vertex_attribute<'a>(
    schema: &'a DefaultGeometrySchema,
    name: &str,
) -> Result<&'a GeometryAttribute, GeometryError> {
    let attributes = &schema.vertex_attributes;
    match name {
        "position" => Some(&attributes.position),
        "normal" => Some(&attributes.normal),
        "uv0" => Some(&attributes.uv0),
        "color" => Some(&attributes.color),
        "region" | "uvRegion" => attributes.region.as_ref(),
        _ => None,
    }
    .ok_or_else(|| GeometryError::UnknownAttribute {
        name: name.to_string(),
    })
}

/// Reads one per-feature attribute of a legacy schema.
///
/// `featureAttributes` entries are typed [`Value`](crate::cmn::Value) rather
/// than `geometryAttribute`, so `valueType` arrives as a free-form string and
/// must be matched against the specification's vocabulary.
fn read_legacy_feature_attribute(
    reader: &mut BufferReader<'_>,
    schema: &DefaultGeometrySchema,
    name: &str,
    feature_count: usize,
) -> Result<GeometryValues, GeometryError> {
    let attributes = &schema.feature_attributes;
    let value = match name {
        "id" => attributes.id.as_ref(),
        "faceRange" => attributes.face_range.as_ref(),
        _ => None,
    }
    .ok_or_else(|| GeometryError::UnknownAttribute {
        name: name.to_string(),
    })?;

    let value_type =
        parse_value_type(&value.value_type).ok_or_else(|| GeometryError::UnknownAttribute {
            name: value.value_type.to_string(),
        })?;
    let component = value.values_per_element.unwrap_or(1) as usize;
    read_n(
        reader,
        value_type,
        component * feature_count,
        "featureAttribute",
    )
}

/// Parses a `valueType` string from a legacy `value` declaration.
fn parse_value_type(name: &str) -> Option<ValueType> {
    Some(match name {
        "UInt8" => ValueType::UInt8,
        "UInt16" => ValueType::UInt16,
        "UInt32" => ValueType::UInt32,
        "UInt64" => ValueType::UInt64,
        "Int8" => ValueType::UInt8,
        "Int16" => ValueType::Int16,
        "Int32" => ValueType::Int32,
        "Int64" => ValueType::Int64,
        "Float32" => ValueType::Float32,
        "Float64" => ValueType::Float64,
        _ => return None,
    })
}

fn store_vertex_attribute(
    geometry: &mut Geometry,
    name: &str,
    values: GeometryValues,
) -> Result<(), GeometryError> {
    match name {
        "position" => geometry.position = Some(values),
        "normal" => geometry.normal = Some(values),
        "uv0" => geometry.uv0 = Some(values),
        "color" => geometry.color = Some(values),
        "region" | "uvRegion" => geometry.uv_region = Some(values),
        _ => {
            return Err(GeometryError::UnknownAttribute {
                name: name.to_string(),
            });
        }
    }
    Ok(())
}

fn store_feature_attribute(
    geometry: &mut Geometry,
    name: &str,
    values: GeometryValues,
) -> Result<(), GeometryError> {
    match name {
        "id" => geometry.feature_id = Some(values),
        "faceRange" => geometry.face_range = Some(values),
        _ => {
            return Err(GeometryError::UnknownAttribute {
                name: name.to_string(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmn::{
        CompressedAttributes, CompressedAttributesEncoding, GeometryColor, GeometryColorType,
        GeometryFaceRange, GeometryFaceRangeType, GeometryNormal, GeometryNormalType,
        GeometryPosition, GeometryPositionType, GeometryUv, GeometryUvType,
    };

    fn f32s(values: &[f32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    fn position_only() -> GeometryBuffer {
        GeometryBuffer {
            position: Some(GeometryPosition {
                component: 3,
                r#type: GeometryPositionType::Float32,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn decodes_positions() {
        let bytes = f32s(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let geometry =
            decode_geometry(&position_only(), &bytes, GeometryCounts::vertices(2)).expect("decode");
        assert_eq!(
            geometry.position.unwrap(),
            GeometryValues::F32(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
        );
        assert_eq!(geometry.vertex_count, 2);
    }

    #[test]
    fn honours_offset_header() {
        let definition = GeometryBuffer {
            offset: Some(8),
            ..position_only()
        };
        let mut bytes = vec![0xAA; 8];
        bytes.extend(f32s(&[1.0, 2.0, 3.0]));
        let geometry =
            decode_geometry(&definition, &bytes, GeometryCounts::vertices(1)).expect("decode");
        assert_eq!(
            geometry.position.unwrap().as_f32(),
            Some(&[1.0, 2.0, 3.0][..])
        );
    }

    #[test]
    fn reads_attributes_in_the_specified_fixed_order() {
        // position(f32x3) then normal(f32x3) then uv0(f32x2) then color(u8x4).
        // If the decoder read these in any other order the values would be
        // misinterpreted, so distinct values pin the ordering down.
        let definition = GeometryBuffer {
            position: Some(GeometryPosition {
                component: 3,
                r#type: GeometryPositionType::Float32,
                ..Default::default()
            }),
            normal: Some(GeometryNormal {
                component: 3,
                r#type: GeometryNormalType::Float32,
                ..Default::default()
            }),
            uv0: Some(GeometryUv {
                component: 2,
                r#type: GeometryUvType::Float32,
                ..Default::default()
            }),
            color: Some(GeometryColor {
                component: 4,
                r#type: GeometryColorType::UInt8,
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut bytes = f32s(&[1.0, 2.0, 3.0]);
        bytes.extend(f32s(&[0.0, 0.0, 1.0]));
        bytes.extend(f32s(&[0.25, 0.75]));
        bytes.extend_from_slice(&[10, 20, 30, 255]);

        let geometry =
            decode_geometry(&definition, &bytes, GeometryCounts::vertices(1)).expect("decode");
        assert_eq!(
            geometry.position.unwrap().as_f32(),
            Some(&[1.0, 2.0, 3.0][..])
        );
        assert_eq!(
            geometry.normal.unwrap().as_f32(),
            Some(&[0.0, 0.0, 1.0][..])
        );
        assert_eq!(geometry.uv0.unwrap().as_f32(), Some(&[0.25, 0.75][..]));
        assert_eq!(
            geometry.color.unwrap(),
            GeometryValues::U8(vec![10, 20, 30, 255])
        );
    }

    #[test]
    fn unaligned_color_block_does_not_break_following_reads() {
        // A 3-component UInt8 colour leaves the cursor 4-byte-unaligned. The
        // per-feature faceRange that follows must still read correctly.
        let definition = GeometryBuffer {
            color: Some(GeometryColor {
                component: 3,
                r#type: GeometryColorType::UInt8,
                ..Default::default()
            }),
            face_range: Some(GeometryFaceRange {
                component: 2,
                r#type: GeometryFaceRangeType::UInt32,
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut bytes = vec![7u8, 8, 9];
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&5u32.to_le_bytes());

        let counts = GeometryCounts {
            vertex_count: 1,
            feature_count: 1,
        };
        let geometry = decode_geometry(&definition, &bytes, counts).expect("decode");
        assert_eq!(geometry.color.unwrap(), GeometryValues::U8(vec![7, 8, 9]));
        // faceRange is inclusive: triangles 0..=5 belong to feature 0.
        assert_eq!(
            geometry.face_range.unwrap(),
            GeometryValues::U32(vec![0, 5])
        );
    }

    #[test]
    fn per_feature_attributes_use_the_feature_count() {
        let definition = GeometryBuffer {
            face_range: Some(GeometryFaceRange {
                component: 2,
                r#type: GeometryFaceRangeType::UInt32,
                ..Default::default()
            }),
            ..Default::default()
        };
        let bytes: Vec<u8> = [0u32, 3, 4, 9]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let counts = GeometryCounts {
            vertex_count: 100,
            feature_count: 2,
        };
        let geometry = decode_geometry(&definition, &bytes, counts).expect("decode");
        assert_eq!(
            geometry.face_range.unwrap(),
            GeometryValues::U32(vec![0, 3, 4, 9])
        );
    }

    #[test]
    fn draco_buffer_is_reported_not_guessed() {
        let definition = GeometryBuffer {
            compressed_attributes: Some(CompressedAttributes {
                encoding: CompressedAttributesEncoding::Draco,
                attributes: Vec::new(),
            }),
            ..Default::default()
        };
        assert_eq!(
            decode_geometry(&definition, &[], GeometryCounts::default()),
            Err(GeometryError::Compressed)
        );
    }

    #[test]
    fn short_buffer_is_an_error() {
        let bytes = f32s(&[1.0, 2.0]);
        assert!(matches!(
            decode_geometry(&position_only(), &bytes, GeometryCounts::vertices(1)),
            Err(GeometryError::UnexpectedEnd { .. })
        ));
    }

    #[test]
    fn trailing_bytes_are_an_error() {
        // A payload longer than the declared layout means the definition and
        // the data disagree; decoding anything from it would be a guess.
        let mut bytes = f32s(&[1.0, 2.0, 3.0]);
        bytes.push(0);
        assert_eq!(
            decode_geometry(&position_only(), &bytes, GeometryCounts::vertices(1)),
            Err(GeometryError::TrailingBytes { trailing: 1 })
        );
    }

    #[test]
    fn encode_round_trips_every_attribute() {
        let definition = GeometryBuffer {
            position: Some(GeometryPosition {
                component: 3,
                r#type: GeometryPositionType::Float32,
                ..Default::default()
            }),
            color: Some(GeometryColor {
                component: 4,
                r#type: GeometryColorType::UInt8,
                ..Default::default()
            }),
            face_range: Some(GeometryFaceRange {
                component: 2,
                r#type: GeometryFaceRangeType::UInt32,
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut bytes = f32s(&[1.0, 2.0, 3.0]);
        bytes.extend_from_slice(&[1, 2, 3, 4]);
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&2u32.to_le_bytes());

        let counts = GeometryCounts {
            vertex_count: 1,
            feature_count: 1,
        };
        let geometry = decode_geometry(&definition, &bytes, counts).expect("decode");
        assert_eq!(encode_geometry(&definition, &geometry).unwrap(), bytes);
    }

    #[test]
    fn decodes_a_legacy_default_geometry_schema_buffer() {
        use crate::cmn::{
            DefaultGeometrySchema, FeatureAttribute, HeaderAttribute, Value, VertexAttribute,
        };

        // Legacy buffers carry their counts in the header rather than taking
        // them from the node page.
        let schema = DefaultGeometrySchema {
            topology: DefaultGeometrySchemaTopology::PerAttributeArray,
            header: vec![
                HeaderAttribute {
                    property: "vertexCount".into(),
                    r#type: HeaderAttributeType::UInt32,
                },
                HeaderAttribute {
                    property: "featureCount".into(),
                    r#type: HeaderAttributeType::UInt32,
                },
            ],
            ordering: vec!["position".into(), "color".into()],
            vertex_attributes: VertexAttribute {
                position: GeometryAttribute {
                    value_type: GeometryAttributeValueType::Float32,
                    values_per_element: 3,
                    byte_offset: None,
                },
                color: GeometryAttribute {
                    value_type: GeometryAttributeValueType::UInt8,
                    values_per_element: 4,
                    byte_offset: None,
                },
                ..Default::default()
            },
            feature_attribute_order: vec!["id".into(), "faceRange".into()],
            feature_attributes: FeatureAttribute {
                id: Some(Value {
                    value_type: "UInt64".into(),
                    values_per_element: Some(1),
                    ..Default::default()
                }),
                face_range: Some(Value {
                    value_type: "UInt32".into(),
                    values_per_element: Some(2),
                    ..Default::default()
                }),
            },
            ..Default::default()
        };

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&2u32.to_le_bytes()); // vertexCount
        bytes.extend_from_slice(&1u32.to_le_bytes()); // featureCount
        bytes.extend(f32s(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        bytes.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
        bytes.extend_from_slice(&7u64.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&1u32.to_le_bytes());

        let geometry = decode_legacy_geometry(&schema, &bytes).expect("decode");
        assert_eq!(geometry.vertex_count, 2);
        assert_eq!(geometry.feature_count, 1);
        assert_eq!(
            geometry.position.unwrap(),
            GeometryValues::F32(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
        );
        assert_eq!(
            geometry.color.unwrap(),
            GeometryValues::U8(vec![1, 2, 3, 4, 5, 6, 7, 8])
        );
        // `id` is UInt64 per feature; a narrower read would desynchronise the
        // faceRange that follows.
        assert_eq!(geometry.feature_id.unwrap(), GeometryValues::U64(vec![7]));
        assert_eq!(
            geometry.face_range.unwrap(),
            GeometryValues::U32(vec![0, 1])
        );
    }

    #[test]
    fn encode_preserves_the_offset_header_length() {
        let definition = GeometryBuffer {
            offset: Some(8),
            ..position_only()
        };
        let geometry = Geometry {
            position: Some(GeometryValues::F32(vec![1.0, 2.0, 3.0])),
            vertex_count: 1,
            ..Default::default()
        };
        let encoded = encode_geometry(&definition, &geometry).unwrap();
        assert_eq!(encoded.len(), 8 + 12);
        let decoded =
            decode_geometry(&definition, &encoded, GeometryCounts::vertices(1)).expect("decode");
        assert_eq!(decoded.position, geometry.position);
    }
}

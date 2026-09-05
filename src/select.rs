//! Choosing which geometry buffer of a geometry definition to fetch.
//!
//! I3S does not leave this to a heuristic. `geometryDefinition.cmn.md`
//! fixes the array positions:
//!
//! > For compatibility reasons, _uncompressed_ geometry buffer is always
//! > required and must be first (i.e. `geometryBuffers[0]`), so array
//! > length must be 1 or 2
//!
//! So `geometryBuffers[0]` is the uncompressed buffer and, when present,
//! `geometryBuffers[1]` is the compressed (Draco) alternative. Scoring the
//! buffers by how many attributes they declare — as an earlier revision of
//! this module did — can only ever reproduce that rule by accident, and
//! silently picks the wrong buffer for any producer whose Draco buffer
//! enumerates fewer attributes than its uncompressed one.

use crate::cmn::{GeometryDefinition, GeometryDefinitionTopology, SceneLayerInfo};

/// Which encoding of a geometry definition a client wants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GeometryEncodingPreference {
    /// Take `geometryBuffers[0]`, which the specification requires to be
    /// uncompressed. This is the default because [`crate::decode_geometry`]
    /// cannot decode Draco.
    #[default]
    Uncompressed,
    /// Take the compressed buffer when the definition offers one, falling
    /// back to the uncompressed buffer otherwise. Only useful to a client
    /// that has its own Draco decoder.
    PreferCompressed,
}

/// The buffer chosen within one geometry definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeometryBufferChoice {
    /// Index into [`SceneLayerInfo::geometry_definitions`].
    pub definition: usize,
    /// Index into `GeometryDefinition::geometry_buffers`.
    pub buffer: usize,
    /// Whether the chosen buffer carries `compressedAttributes`, and so
    /// needs a Draco decoder rather than [`crate::decode_geometry`].
    pub compressed: bool,
}

/// Selects a buffer within a single geometry definition.
///
/// Returns `None` only when the definition declares no buffers at all,
/// which the specification forbids but a malformed document may still do.
pub fn select_buffer_in_definition(
    definition: &GeometryDefinition,
    preference: GeometryEncodingPreference,
) -> Option<usize> {
    if definition.geometry_buffers.is_empty() {
        return None;
    }
    match preference {
        GeometryEncodingPreference::Uncompressed => Some(0),
        GeometryEncodingPreference::PreferCompressed => Some(
            definition
                .geometry_buffers
                .iter()
                .position(|buffer| buffer.compressed_attributes.is_some())
                .unwrap_or(0),
        ),
    }
}

/// Selects a geometry buffer for a layer whose meshes do not name a
/// definition.
///
/// A node's `mesh.geometry.definition` normally identifies the definition
/// directly, and callers that have a node should use that index with
/// [`select_buffer_in_definition`] instead. This function exists for the
/// layer-level case: it takes the first triangle-topology definition,
/// since `topology` is documented as necessarily `triangle`.
pub fn select_geometry_buffer(
    layer: &SceneLayerInfo,
    preference: GeometryEncodingPreference,
) -> Option<GeometryBufferChoice> {
    layer
        .geometry_definitions
        .iter()
        .enumerate()
        .find(|(_, definition)| is_triangle(definition))
        .and_then(|(index, definition)| {
            let buffer = select_buffer_in_definition(definition, preference)?;
            Some(GeometryBufferChoice {
                definition: index,
                buffer,
                compressed: definition.geometry_buffers[buffer]
                    .compressed_attributes
                    .is_some(),
            })
        })
}

/// Whether a definition describes triangle meshes.
///
/// `topology` is optional in the schema but the specification admits only
/// `triangle`, so an absent value is read as triangle rather than as a
/// reason to skip the definition.
fn is_triangle(definition: &GeometryDefinition) -> bool {
    definition
        .topology
        .as_ref()
        .is_none_or(|topology| *topology == GeometryDefinitionTopology::Triangle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmn::{
        Attributes, CompressedAttributes, CompressedAttributesEncoding, GeometryBuffer,
        GeometryNormal,
    };

    fn draco() -> GeometryBuffer {
        GeometryBuffer {
            compressed_attributes: Some(CompressedAttributes {
                encoding: CompressedAttributesEncoding::Draco,
                attributes: vec![Attributes::Position],
            }),
            ..Default::default()
        }
    }

    fn layer_with_buffers(buffers: Vec<GeometryBuffer>) -> SceneLayerInfo {
        SceneLayerInfo {
            geometry_definitions: vec![GeometryDefinition {
                topology: Some(GeometryDefinitionTopology::Triangle),
                geometry_buffers: buffers,
            }],
            ..Default::default()
        }
    }

    #[test]
    fn defaults_to_the_uncompressed_buffer() {
        // Even though the Draco buffer is smaller on the wire, it is not
        // what this crate can decode, so it must not be the default.
        let layer = layer_with_buffers(vec![GeometryBuffer::default(), draco()]);
        let choice = select_geometry_buffer(&layer, GeometryEncodingPreference::default()).unwrap();
        assert_eq!(choice.buffer, 0);
        assert!(!choice.compressed);
    }

    #[test]
    fn finds_the_compressed_buffer_on_request() {
        let layer = layer_with_buffers(vec![GeometryBuffer::default(), draco()]);
        let choice =
            select_geometry_buffer(&layer, GeometryEncodingPreference::PreferCompressed).unwrap();
        assert_eq!(choice.buffer, 1);
        assert!(choice.compressed);
    }

    #[test]
    fn falls_back_when_no_compressed_buffer_exists() {
        let layer = layer_with_buffers(vec![GeometryBuffer::default()]);
        let choice =
            select_geometry_buffer(&layer, GeometryEncodingPreference::PreferCompressed).unwrap();
        assert_eq!(choice.buffer, 0);
        assert!(!choice.compressed);
    }

    #[test]
    fn richer_attribute_lists_do_not_win() {
        // The old scoring heuristic would have chosen buffer 1 here. The
        // specification says buffer 0 is the uncompressed one, full stop.
        let layer = layer_with_buffers(vec![
            GeometryBuffer::default(),
            GeometryBuffer {
                normal: Some(GeometryNormal::default()),
                uv0: Some(Default::default()),
                ..Default::default()
            },
        ]);
        let choice = select_geometry_buffer(&layer, GeometryEncodingPreference::default()).unwrap();
        assert_eq!(choice.buffer, 0);
    }

    #[test]
    fn absent_topology_is_treated_as_triangle() {
        let layer = SceneLayerInfo {
            geometry_definitions: vec![GeometryDefinition {
                topology: None,
                geometry_buffers: vec![GeometryBuffer::default()],
            }],
            ..Default::default()
        };
        assert!(select_geometry_buffer(&layer, GeometryEncodingPreference::default()).is_some());
    }

    #[test]
    fn returns_none_without_definitions() {
        let layer = SceneLayerInfo {
            geometry_definitions: vec![],
            ..Default::default()
        };
        assert_eq!(
            select_geometry_buffer(&layer, GeometryEncodingPreference::default()),
            None
        );
    }
}

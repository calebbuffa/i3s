use crate::{GeometryCounts, cmn::GeometryBuffer};

/// Native layout and counts required to interpret a geometry buffer.
#[derive(Clone)]
pub struct GeometryDescriptor {
    /// The selected buffer definition.
    pub buffer: GeometryBuffer,
    /// Counts declared by the node page.
    pub counts: GeometryCounts,
}

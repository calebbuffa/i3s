//! Stable I3S identifiers.
//!
//! These name the entities the specification defines. They carry no packaging
//! or compilation policy; manifest and emission decisions belong to the
//! application that builds packages.

/// A stable I3S node identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

/// A stable I3S node-page identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodePageId(pub u64);

/// A stable I3S node-scoped resource identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(pub u32);

/// A Building Scene Layer sublayer identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SublayerId(pub u32);

/// An I3S attribute storage key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttributeKey(pub String);

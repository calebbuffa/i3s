//! Typed logical locations within an I3S scene layer.

use std::fmt;

use crate::{Error, Source};

/// A validated relative path within an SLPK or eSLPK package.
///
/// Paths always use forward slashes and cannot escape a package root.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourcePath(String);

impl ResourcePath {
    /// Validates a package-relative path.
    pub fn new(path: impl AsRef<str>) -> Result<Self, Error> {
        let path = path.as_ref();
        if path.is_empty() {
            return Err(Error::InvalidResourcePath {
                path: path.into(),
                reason: "path is empty",
            });
        }
        if path.starts_with('/') || path.starts_with('\\') {
            return Err(Error::InvalidResourcePath {
                path: path.into(),
                reason: "path is absolute",
            });
        }
        if path.contains('\\') {
            return Err(Error::InvalidResourcePath {
                path: path.into(),
                reason: "path uses a platform separator",
            });
        }
        if path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err(Error::InvalidResourcePath {
                path: path.into(),
                reason: "path contains an empty or traversal segment",
            });
        }
        Ok(Self(path.into()))
    }

    /// Returns the normalized package-relative path.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourcePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A typed logical resource location.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResourceLocation {
    /// The root layer document.
    Layer,
    /// A node page.
    NodePage(u64),
    /// A geometry buffer.
    Geometry {
        /// Node resource identifier.
        resource: u32,
        /// Index of the buffer within the node's geometry definition.
        buffer: usize,
    },
    /// An attribute buffer, addressed by its bare numeric key.
    Attribute {
        /// Node resource identifier.
        resource: u32,
        /// The attribute's numeric key.
        key: usize,
    },
    /// A texture resource.
    Texture {
        /// Node resource identifier.
        resource: u32,
        /// Format-defined texture name.
        name: String,
        /// File extension in a package.
        extension: String,
    },
    /// Per-field statistics.
    Statistics {
        /// The attribute's numeric key.
        key: usize,
    },
    /// Building-layer summary statistics.
    StatisticsSummary,
    /// Legacy feature data.
    FeatureData {
        /// Node resource identifier.
        resource: u32,
        /// Index of the feature-data document within the node.
        feature: usize,
    },
    /// A legacy node document.
    LegacyNode(u32),
    /// A legacy shared resource document.
    SharedResource(u32),
    /// Package metadata.
    Metadata,
    /// Optional SLPK direct-lookup index.
    HashIndex,
    /// An explicitly validated package-relative resource.
    Package(ResourcePath),
}

impl ResourceLocation {
    /// Resolves this logical location through a source's transport naming rules.
    pub fn resolve(&self, source: &Source) -> String {
        match self {
            Self::Layer => source.layer(),
            Self::NodePage(id) => source.node_page(*id),
            Self::Geometry { resource, buffer } => source.geometry(*resource, *buffer),
            Self::Attribute { resource, key } => source.attribute(*resource, *key),
            Self::Texture {
                resource,
                name,
                extension,
            } => source.texture(*resource, name, extension),
            Self::Statistics { key } => source.statistics(*key),
            Self::StatisticsSummary => source.statistics_summary(),
            Self::FeatureData { resource, feature } => source.feature_data(*resource, *feature),
            Self::LegacyNode(id) => source.node(*id),
            Self::SharedResource(id) => source.shared(*id),
            Self::Metadata => "metadata.json".into(),
            Self::HashIndex => "@specialIndexFileHASH128@".into(),
            Self::Package(path) => path.to_string(),
        }
    }
}

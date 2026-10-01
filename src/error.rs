//! Error type for I3S loading.

/// An error raised while loading an I3S scene layer.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Failed to fetch a resource.
    #[error("failed to fetch {uri}: {source}")]
    Fetch {
        /// The URI that could not be fetched.
        uri: String,
        /// The underlying transport error.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// A JSON document failed to parse.
    #[error("failed to parse {uri}: {message}")]
    Parse {
        /// The URI of the document that failed to parse.
        uri: String,
        /// The underlying parse failure.
        message: String,
    },

    /// The layer document has no `nodePages`, so it uses the legacy
    /// per-node `3dNodeIndexDocument` model that this loader does not
    /// implement.
    #[error("layer {layer_id} has no nodePages definition")]
    NoNodePages {
        /// The layer that lacks a node-page definition.
        layer_id: u32,
    },

    /// `nodePages.nodesPerPage` was zero or negative.
    #[error("layer {layer_id} declares an invalid nodesPerPage of {nodes_per_page}")]
    InvalidNodesPerPage {
        /// The offending layer.
        layer_id: u32,
        /// The declared value.
        nodes_per_page: u32,
    },

    /// A node index referenced by the tree was not present in the page that
    /// should contain it.
    #[error("node {node_index} is missing from node page {page_id}")]
    MissingNode {
        /// The node index that could not be resolved.
        node_index: u64,
        /// The page that should have held it.
        page_id: u64,
    },

    /// A package resource path was invalid.
    #[error("invalid I3S package resource path `{path}`: {reason}")]
    InvalidResourcePath {
        /// The rejected path.
        path: String,
        /// Why it was rejected.
        reason: &'static str,
    },

    /// A local package operation failed.
    #[error("I3S package I/O failed at {path}: {source}")]
    Io {
        /// The affected filesystem path.
        path: String,
        /// The underlying filesystem error.
        #[source]
        source: std::io::Error,
    },

    /// A ZIP SLPK could not be read or written.
    #[error("SLPK archive error: {message}")]
    Archive {
        /// The archive error.
        message: String,
    },

    /// A rewritten scene layer failed format-consistency validation.
    #[error("invalid scene-layer rewrite: {message}")]
    InvalidRewrite {
        /// The failed invariant.
        message: String,
    },

    /// A CPU job submitted to a caller-provided [`hiera::Spawner`] failed.
    #[error("spawned parse job failed: {0}")]
    Spawn(#[source] hiera::SpawnError),
}

impl Error {
    pub(crate) fn fetch(
        uri: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        Self::Fetch {
            uri: uri.into(),
            source,
        }
    }

    /// Creates a contextual error for a resource whose bytes could not be
    /// interpreted as the document the specification requires there.
    ///
    /// Applications that build on this crate (package compilers, converters,
    /// validators) construct the same error shape when their own encoding
    /// step fails, so the variant is constructible from outside.
    pub fn parse(uri: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Parse {
            uri: uri.into(),
            message: message.into(),
        }
    }

    /// Creates a contextual error for an application-owned package I/O adapter.
    ///
    /// I3S core does not perform filesystem/archive I/O itself; applications
    /// use this when implementing [`crate::SceneLayerStore`].
    pub fn io(path: impl Into<String>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }

    /// Create a spawn error from a failed [`hiera::Spawner`] job.
    pub(crate) fn spawn(source: hiera::SpawnError) -> Self {
        Self::Spawn(source)
    }
}

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

    pub(crate) fn parse(uri: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Parse {
            uri: uri.into(),
            message: message.into(),
        }
    }
}

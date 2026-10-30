//! Error type for interpreting I3S documents and resources.

/// An error raised while interpreting or composing I3S specification data.
///
/// Transport, storage, and execution errors belong to the layer that performs
/// them; this type covers only facts the specification defines.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// A JSON document failed to parse.
    #[error("failed to parse {uri}: {message}")]
    Parse {
        /// The URI of the document that failed to parse.
        uri: String,
        /// The underlying parse failure.
        message: String,
    },

    /// The layer document has no node-page collection for a requested page.
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

    /// A rewritten scene layer failed format-consistency validation.
    #[error("invalid scene-layer rewrite: {message}")]
    InvalidRewrite {
        /// The failed invariant.
        message: String,
    },
}

impl Error {
    /// Creates a contextual error for a resource whose bytes could not be
    /// interpreted as the document the specification requires there.
    pub fn parse(uri: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Parse {
            uri: uri.into(),
            message: message.into(),
        }
    }
}

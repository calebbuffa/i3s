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

    /// A JSON document or value failed to parse or serialize.
    #[error("failed to process JSON at {uri}: {source}")]
    Json {
        /// The URI or resource context.
        uri: String,
        /// The underlying JSON failure.
        #[source]
        source: serde_json::Error,
    },

    /// An attribute buffer ended before its declared values were available.
    #[error("attribute buffer truncated")]
    AttributeTruncated,
    /// An attribute descriptor uses an unknown value type.
    #[error("unknown attribute value type: {0}")]
    AttributeUnknownValueType(String),
    /// An attribute string is not valid UTF-8.
    #[error("invalid UTF-8 at feature {feature_index}: {error}")]
    AttributeInvalidUtf8 {
        /// Index of the offending feature within the node.
        feature_index: usize,
        /// The underlying UTF-8 decoding failure.
        #[source]
        error: std::str::Utf8Error,
    },
    /// An attribute descriptor has no `attributeValues` section.
    #[error("attributeStorageInfo has no attributeValues descriptor")]
    AttributeMissingValues,
    /// An attribute count exceeds the decoder's safety limit.
    #[error("attribute count {count} exceeds maximum allowed ({max})")]
    AttributeCountTooLarge {
        /// The count declared by the binary header.
        count: usize,
        /// The largest count this crate will allocate for.
        max: usize,
    },
    /// Attribute buffer size arithmetic overflowed.
    #[error("attribute buffer arithmetic overflow")]
    AttributeOverflow,
    /// An attribute value does not match the declared I3S storage type.
    #[error("attribute value does not match declared type `{0}`")]
    AttributeTypeMismatch(String),

    /// A geometry buffer ended before the declared layout was satisfied.
    #[error("geometry buffer ended early while reading `{attribute}`")]
    GeometryUnexpectedEnd {
        /// The attribute being read when the buffer ran out.
        attribute: &'static str,
    },
    /// A geometry buffer uses Draco compression.
    #[error("geometry buffer uses `compressedAttributes` (Draco) encoding")]
    GeometryCompressed,
    /// Bytes remained after the declared geometry layout was consumed.
    #[error("geometry buffer has {trailing} trailing byte(s) after the declared layout")]
    GeometryTrailingBytes {
        /// Number of bytes left over.
        trailing: usize,
    },
    /// A geometry layout declares an unsupported topology.
    #[error("unsupported geometry topology `{topology}`")]
    GeometryUnsupportedTopology {
        /// The declared topology.
        topology: &'static str,
    },
    /// A legacy geometry schema names an undefined attribute.
    #[error("unknown legacy geometry attribute `{name}`")]
    GeometryUnknownAttribute {
        /// The offending name from `ordering` or `featureAttributeOrder`.
        name: String,
    },
    /// A legacy geometry header lacks `vertexCount`.
    #[error("legacy geometry header has no `vertexCount` property")]
    GeometryMissingVertexCount,
    /// An edited geometry does not match its declared buffer layout.
    #[error("geometry attribute `{attribute}` does not match the declared layout")]
    GeometryLayoutMismatch {
        /// The declared attribute that failed validation.
        attribute: &'static str,
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

    /// Creates an error for a JSON resource while retaining its source.
    pub fn json(uri: impl Into<String>, source: serde_json::Error) -> Self {
        Self::Json {
            uri: uri.into(),
            source,
        }
    }
}

impl From<crate::binary::UnexpectedEndOfData> for Error {
    fn from(_: crate::binary::UnexpectedEndOfData) -> Self {
        Self::AttributeTruncated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_error_retains_serde_source() {
        let source = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
        let error = Error::json("layer.json", source);
        assert!(
            std::error::Error::source(&error)
                .is_some_and(|source| source.downcast_ref::<serde_json::Error>().is_some())
        );
    }
}

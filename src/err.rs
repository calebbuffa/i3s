use thiserror::Error;

#[derive(Debug, Error)]
pub enum I3SError {
    #[error("Failed to decompress data: {0}")]
    DecompressionError(String),

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Invalid URI: {0}")]
    InvalidUri(String),

    #[error("Other error: {0}")]
    Other(String),
}
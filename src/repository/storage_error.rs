use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to access storage: {0}")]
    AccessError(String),

    #[error("Failed to serialize or deserialize data: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Key not found: {0}")]
    NotFound(String),

    #[error("Unknown error occurred: {0}")]
    Unknown(String),
}

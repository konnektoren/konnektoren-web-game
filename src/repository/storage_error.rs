use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to access storage: {0}")]
    AccessError(String),

    #[error("Data not found")]
    NotFound,

    #[error("Unknown error occurred")]
    Unknown,
}
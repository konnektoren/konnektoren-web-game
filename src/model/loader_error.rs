use gloo::storage::errors::StorageError;

#[derive(Debug)]
pub enum LoaderError {
    YamlError(serde_yaml::Error),
    IoError(std::io::Error),
    StorageError(StorageError),
    InvalidData(String),
}

impl From<serde_yaml::Error> for LoaderError {
    fn from(err: serde_yaml::Error) -> Self {
        LoaderError::YamlError(err)
    }
}

impl From<std::io::Error> for LoaderError {
    fn from(err: std::io::Error) -> Self {
        LoaderError::IoError(err)
    }
}

impl From<StorageError> for LoaderError {
    fn from(err: StorageError) -> Self {
        LoaderError::StorageError(err)
    }
}

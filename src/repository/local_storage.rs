use super::storage::Storage;
use super::storage_error::StorageError;
use async_trait::async_trait;
use gloo::storage::{LocalStorage as GlooLocalStorage, Storage as GlooStorage};

pub struct LocalStorage {
    key_prefix: String,
}

impl LocalStorage {
    pub fn new(key_prefix: &str) -> Self {
        LocalStorage {
            key_prefix: key_prefix.to_string(),
        }
    }

    fn prefixed_key(&self, key: &str) -> String {
        format!("{}:{}", self.key_prefix, key)
    }
}

#[async_trait]
impl Storage for LocalStorage {
    async fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        let prefixed_key = self.prefixed_key(key);
        match GlooLocalStorage::get(&prefixed_key) {
            Ok(value) => Ok(Some(value)),
            Err(gloo::storage::errors::StorageError::KeyNotFound(_)) => Ok(None),
            Err(e) => Err(StorageError::AccessError(e.to_string())),
        }
    }

    async fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        let prefixed_key = self.prefixed_key(key);
        GlooLocalStorage::set(&prefixed_key, value)
            .map_err(|e| StorageError::AccessError(e.to_string()))
    }

    async fn remove(&self, key: &str) -> Result<(), StorageError> {
        let prefixed_key = self.prefixed_key(key);
        GlooLocalStorage::delete(&prefixed_key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_local_storage() {
        let storage = LocalStorage::new("test");
        storage.set("key", "value").await.unwrap();
        assert_eq!(storage.get("key").await.unwrap(), Some("value".to_string()));
        storage.remove("key").await.unwrap();
        assert_eq!(storage.get("key").await.unwrap(), None);
    }
}
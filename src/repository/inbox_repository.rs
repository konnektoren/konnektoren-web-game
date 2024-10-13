use super::repository::Repository;
use super::repository_error::RepositoryError;
use super::storage::Storage;
use crate::model::Inbox;
use async_trait::async_trait;
use serde_json;

pub struct InboxRepository<S: Storage> {
    storage: S,
}

impl<S: Storage> InboxRepository<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<S: Storage + Send + Sync> Repository<Inbox> for InboxRepository<S> {
    async fn save(&self, key: &str, inbox: &Inbox) -> Result<(), RepositoryError> {
        let serialized =
            serde_json::to_string(inbox).map_err(RepositoryError::SerializationError)?;
        self.storage
            .set(key, &serialized)
            .await
            .map_err(|e| RepositoryError::StorageError(e.to_string()))
    }

    async fn get(&self, key: &str) -> Result<Option<Inbox>, RepositoryError> {
        match self.storage.get(key).await {
            Ok(Some(serialized)) => {
                let inbox = serde_json::from_str(&serialized)
                    .map_err(RepositoryError::SerializationError)?;
                Ok(Some(inbox))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(RepositoryError::StorageError(e.to_string())),
        }
    }

    async fn delete(&self, key: &str) -> Result<(), RepositoryError> {
        self.storage
            .remove(key)
            .await
            .map_err(|e| RepositoryError::StorageError(e.to_string()))
    }
}

impl<S: Storage + Send + Sync> InboxRepository<S> {
    pub async fn merge_inbox(&self, key: &str, other: &Inbox) -> Result<(), RepositoryError> {
        let mut current = self.get(key).await?.unwrap_or_default();
        current.merge(other);
        self.save(key, &current).await
    }

    pub async fn add_message(
        &self,
        key: &str,
        message: yew_chat::prelude::Message,
    ) -> Result<(), RepositoryError> {
        let mut inbox = self.get(key).await?.unwrap_or_default();
        inbox.messages.push(message);
        inbox.messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        self.save(key, &inbox).await
    }

    pub async fn mark_as_read(&self, key: &str, message_id: &str) -> Result<(), RepositoryError> {
        let mut inbox = self.get(key).await?.unwrap_or_default();
        let read_messages = inbox.read_messages.get_or_insert_with(Vec::new);
        if !read_messages.contains(&message_id.to_string()) {
            read_messages.push(message_id.to_string());
        }
        self.save(key, &inbox).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::memory_storage::MemoryStorage;
    use wasm_bindgen_test::*;
    use yew_chat::prelude::Message;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_inbox_repository() {
        let storage = MemoryStorage::default();
        let repo = InboxRepository::new(storage);
        let key = "test_inbox";

        // Test saving and retrieving an inbox
        let inbox = Inbox {
            messages: vec![Message {
                id: Some("1".to_string()),
                content: "Test message".to_string(),
                ..Default::default()
            }],
            read_messages: Some(vec!["1".to_string()]),
        };
        repo.save(key, &inbox).await.unwrap();

        let retrieved_inbox = repo.get(key).await.unwrap().unwrap();
        assert_eq!(retrieved_inbox.messages.len(), 1);
        assert_eq!(retrieved_inbox.messages[0].id, Some("1".to_string()));

        // Test merging inboxes
        let other_inbox = Inbox {
            messages: vec![Message {
                id: Some("2".to_string()),
                content: "Another message".to_string(),
                ..Default::default()
            }],
            read_messages: Some(vec!["2".to_string()]),
        };
        repo.merge_inbox(key, &other_inbox).await.unwrap();

        let merged_inbox = repo.get(key).await.unwrap().unwrap();
        assert_eq!(merged_inbox.messages.len(), 2);
        assert_eq!(merged_inbox.read_messages.as_ref().unwrap().len(), 2);

        // Test adding a message
        let new_message = Message {
            id: Some("3".to_string()),
            content: "New message".to_string(),
            ..Default::default()
        };
        repo.add_message(key, new_message).await.unwrap();

        let updated_inbox = repo.get(key).await.unwrap().unwrap();
        assert_eq!(updated_inbox.messages.len(), 3);
        assert_eq!(updated_inbox.messages[2].id, Some("3".to_string()));

        // Test marking a message as read
        repo.mark_as_read(key, "3").await.unwrap();

        let final_inbox = repo.get(key).await.unwrap().unwrap();
        assert_eq!(final_inbox.read_messages.as_ref().unwrap().len(), 3);
        assert!(final_inbox
            .read_messages
            .as_ref()
            .unwrap()
            .contains(&"3".to_string()));

        // Test deleting the inbox
        repo.delete(key).await.unwrap();
        assert!(repo.get(key).await.unwrap().is_none());
    }
}

use crate::model::ChallengeTypes;
use async_trait::async_trait;
use konnektoren_core::challenges::ChallengeType;
use konnektoren_yew::repository::{Repository, RepositoryError, Storage};

pub const CHALLENGE_TYPES_STORAGE_KEY: &str = "konnektoren_challenge_types";

pub struct ChallengeTypesRepository<S: Storage> {
    storage: S,
}

impl<S: Storage> ChallengeTypesRepository<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<S: Storage + Send + Sync> Repository<ChallengeTypes> for ChallengeTypesRepository<S> {
    async fn save(
        &self,
        key: &str,
        challenge_types: &ChallengeTypes,
    ) -> Result<(), RepositoryError> {
        self.storage
            .set(key, challenge_types)
            .await
            .map_err(|e| RepositoryError::StorageError(e.to_string()))
    }

    async fn get(&self, key: &str) -> Result<Option<ChallengeTypes>, RepositoryError> {
        match self.storage.get(key).await {
            Ok(Some(challenge_types)) => Ok(Some(challenge_types)),
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

impl<S: Storage + Send + Sync> ChallengeTypesRepository<S> {
    pub async fn merge_challenge_types(
        &self,
        key: &str,
        other: &ChallengeTypes,
    ) -> Result<(), RepositoryError> {
        let mut current = self.get(key).await?.unwrap_or_default();
        current.challenges.extend(other.challenges.clone());
        self.save(key, &current).await
    }

    pub async fn add_challenge(
        &self,
        key: &str,
        challenge: ChallengeType,
    ) -> Result<(), RepositoryError> {
        let mut challenge_types = self.get(key).await?.unwrap_or_default();
        challenge_types.add_challenge(challenge);
        self.save(key, &challenge_types).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_core::challenges::ChallengeType;
    use konnektoren_yew::repository::MemoryStorage;

    #[tokio::test]
    async fn test_challenge_types_repository() {
        let storage = MemoryStorage::default();
        let repo = ChallengeTypesRepository::new(storage);
        let key = "test_challenge_types";

        // Test saving and retrieving challenge types
        let challenge_types = ChallengeTypes {
            challenges: vec![ChallengeType::default()],
        };
        repo.save(key, &challenge_types).await.unwrap();

        let retrieved_challenge_types = repo.get(key).await.unwrap().unwrap();
        assert_eq!(retrieved_challenge_types.challenges.len(), 1);

        // Test merging challenge types
        let other_challenge_types = ChallengeTypes {
            challenges: vec![ChallengeType::default()],
        };
        repo.merge_challenge_types(key, &other_challenge_types)
            .await
            .unwrap();

        let merged_challenge_types = repo.get(key).await.unwrap().unwrap();
        assert_eq!(merged_challenge_types.challenges.len(), 2);

        // Test adding a challenge
        let new_challenge = ChallengeType::default();
        repo.add_challenge(key, new_challenge).await.unwrap();

        let updated_challenge_types = repo.get(key).await.unwrap().unwrap();
        assert_eq!(updated_challenge_types.challenges.len(), 3);

        // Test deleting the challenge types
        repo.delete(key).await.unwrap();
        assert!(repo.get(key).await.unwrap().is_none());
    }
}

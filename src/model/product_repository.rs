use gloo::storage::{LocalStorage, Storage};
use konnektoren_core::challenges::{ChallengeConfig, PackageReader};
use konnektoren_core::game::{Game, GamePath};
use konnektoren_core::marketplace::Product;
use konnektoren_yew::repository::{SessionRepositoryTrait, SESSION_STORAGE_KEY};

const GAME_PATH_ID: &str = "custom_level";
const GAME_PATH_NAME: &str = "Custom Level";

pub struct ProductRepository {}

impl ProductRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_game_path() -> GamePath {
        GamePath {
            id: GAME_PATH_ID.to_string(),
            name: GAME_PATH_NAME.to_string(),
            challenges: vec![],
            map: None,
        }
    }

    pub async fn store(&self, product: Product, session_repository: &dyn SessionRepositoryTrait) {
        let session = &mut session_repository
            .get_session(SESSION_STORAGE_KEY)
            .await
            .unwrap()
            .unwrap_or_default();
        let game = &mut session.game_state.game;

        let challenge_config = self.fetch_challenge_config(product.clone(), game).await;

        log::info!("challenge_config: {:?}", challenge_config);

        let custom_game_path = game
            .game_paths
            .iter_mut()
            .find(|game_path| game_path.id == GAME_PATH_ID);

        if let Some(custom_game_path) = custom_game_path {
            if let Some(challenge_config) = challenge_config {
                if !custom_game_path
                    .challenges
                    .iter()
                    .any(|c| c.id == challenge_config.id)
                {
                    custom_game_path.challenges.push(challenge_config);
                }
            }
            self.store_custom_level(custom_game_path);
        } else {
            if let Some(challenge_config) = challenge_config {
                let mut new_custom_game_path = Self::new_game_path();
                new_custom_game_path.challenges.push(challenge_config);
                self.store_custom_level(&new_custom_game_path);
                game.game_paths.push(new_custom_game_path);
            }
        }

        session_repository
            .save_session(SESSION_STORAGE_KEY, session)
            .await
            .unwrap();
    }

    pub fn store_custom_level(&self, game_path: &GamePath) {
        match LocalStorage::set(GAME_PATH_ID, game_path) {
            Ok(_) => {
                log::info!("Custom level saved to storage");
            }
            Err(e) => {
                log::error!("Error saving custom level to storage: {:?}", e);
            }
        }
    }

    pub fn fetch_custom_level(&self) -> Option<GamePath> {
        #[cfg(target_arch = "wasm32")]
        match LocalStorage::get(GAME_PATH_ID) {
            Ok(Some(game_path)) => {
                let game_path: GamePath = game_path;
                Some(game_path)
            }
            Ok(None) => Some(Self::new_game_path()),
            Err(e) => {
                log::error!("Error loading custom level from storage: {:?}", e);
                Some(Self::new_game_path())
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Some(Self::new_game_path())
        }
    }

    pub async fn fetch_challenge_config(
        &self,
        product: Product,
        game: &Game,
    ) -> Option<ChallengeConfig> {
        match product.path {
            Some(data_path) => {
                log::debug!("Downloading package from: {}", data_path);
                match PackageReader::download(&data_path).await {
                    Ok(package_data) => {
                        log::debug!("Package data downloaded");
                        match PackageReader::read(&package_data) {
                            Ok(package) => {
                                log::debug!("Package loaded successfully");
                                Some(package.get_challenge_config().unwrap_or_default())
                            }
                            Err(e) => {
                                log::error!("Error reading package: {:?}", e);
                                None
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Error downloading package: {:?}", e);
                        None
                    }
                }
            }
            None => product.id.and_then(|product_id| {
                game.game_paths
                    .iter()
                    .flat_map(|game_path| game_path.challenges.iter())
                    .find(|challenge_config| challenge_config.id == product_id)
                    .cloned()
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_core::game::GamePath;

    #[tokio::test]
    async fn test_fetch_challenge_config() {
        let product = Product {
            id: Some("konnektoren".to_string()),
            name: "Konnektoren".to_string(),
            description: "Konnektoren".to_string(),
            price: None,
            image: None,
            tags: vec![],
            path: None,
        };

        let game = Game {
            game_paths: vec![GamePath {
                id: "konnektoren".to_string(),
                name: "Konnektoren".to_string(),
                challenges: vec![ChallengeConfig {
                    id: "konnektoren".to_string(),
                    name: "Konnektoren".to_string(),
                    description: "Konnektoren".to_string(),
                    tasks: 2.into(),
                    unlock_points: 10,
                    challenge: "konnektoren".to_string(),
                    variant: None,
                    position: None,
                }],
                map: None,
            }],
            ..Default::default()
        };

        let product_repository = ProductRepository::new();
        let challenge_config = product_repository
            .fetch_challenge_config(product, &game)
            .await;

        assert_eq!(challenge_config.unwrap().id, "konnektoren");
    }
}

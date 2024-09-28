use crate::model::WebSession;
use gloo::storage::{LocalStorage, Storage};
use konnektoren_core::challenges::ChallengeConfig;
use konnektoren_core::game::{Game, GamePath};
use konnektoren_core::marketplace::Product;

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

    pub async fn store(&self, product: Product) {
        let mut web_session = WebSession::default();
        web_session.load().unwrap_or_default();

        let session = &mut web_session.session;
        let game = &mut session.game_state.game;

        let challenge_config = self.fetch_challenge_config(product.clone(), game).await;

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

        web_session.save().unwrap();
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
        let challenge_config = match product.path {
            Some(_data_path) => None,
            None => {
                if let Some(product_id) = product.id {
                    let challenge_config = game
                        .game_paths
                        .iter()
                        .flat_map(|game_path| game_path.challenges.iter())
                        .find(|challenge_config| challenge_config.id == product_id);

                    challenge_config
                } else {
                    None
                }
            }
        };
        challenge_config.cloned()
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
                    tasks: 2,
                    unlock_points: 10,
                    challenge: "konnektoren".to_string(),
                    variant: None,
                    position: None,
                }],
                map: None,
            }],
            challenge_factory: Default::default(),
            challenge_history: Default::default(),
        };

        let product_repository = ProductRepository::new();
        let challenge_config = product_repository
            .fetch_challenge_config(product, &game)
            .await;

        assert_eq!(challenge_config.unwrap().id, "konnektoren");
    }
}

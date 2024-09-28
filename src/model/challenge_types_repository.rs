use crate::model::challenge_types::ChallengeTypes;
use crate::model::WebSession;
use gloo::storage::{LocalStorage, Storage};
use konnektoren_core::challenges::{ChallengeType, PackageReader};
use konnektoren_core::marketplace::Product;

const CHALLENGES_ID: &str = "challenges";

pub struct ChallengeTypesRepository {}

impl ChallengeTypesRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn store(&self, product: Product) {
        let challenge_type = self.fetch_challenge(&product).await;

        if let Some(challenge_type) = challenge_type {
            let mut custom_challenges = self.fetch_challenges().unwrap_or_default();
            custom_challenges.add_challenge(challenge_type);
            self.store_challenges(&custom_challenges);
        }
    }

    pub fn store_challenges(&self, custom_challenges: &ChallengeTypes) {
        match LocalStorage::set(CHALLENGES_ID, custom_challenges) {
            Ok(_) => log::info!("Challenges stored successfully"),
            Err(e) => log::error!("Error storing challenges: {:?}", e),
        }
    }

    pub fn fetch_challenges(&self) -> Option<ChallengeTypes> {
        match LocalStorage::get(CHALLENGES_ID) {
            Ok(Some(challenges)) => Some(challenges),
            Ok(None) => {
                log::info!("No challenges found");
                None
            }
            Err(e) => {
                log::error!("Error loading challenges: {:?}", e);
                None
            }
        }
    }

    pub async fn fetch_challenge(&self, product: &Product) -> Option<ChallengeType> {
        match &product.path {
            Some(data_path) => match PackageReader::download(&data_path).await {
                Ok(package_data) => match PackageReader::read(&package_data) {
                    Ok(package) => {
                        let mut challenge = package.get_custom_challenge()?;
                        challenge.package_url = Some(data_path.clone());
                        Some(ChallengeType::Custom(challenge))
                    }
                    Err(e) => {
                        log::error!("Error reading package: {:?}", e);
                        None
                    }
                },
                Err(e) => {
                    log::error!("Error downloading package: {:?}", e);
                    None
                }
            },
            None => {
                let mut web_session = WebSession::default();
                web_session.load().unwrap_or_default();

                let session = &mut web_session.session;
                let game = &mut session.game_state.game;
                game.challenge_factory
                    .challenge_types
                    .iter()
                    .find(|challenge_type| {
                        product
                            .id
                            .as_ref()
                            .map_or(false, |id| id == challenge_type.id())
                    })
                    .cloned()
            }
        }
    }
}

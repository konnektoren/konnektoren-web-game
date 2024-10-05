use crate::model::{GameLoader, LoaderError};
use gloo::storage::{LocalStorage, Storage};
use konnektoren_core::game::Game;
use konnektoren_core::session::Session;

#[derive(Debug, Clone)]
pub struct WebSession {
    pub id: String,
    pub session: Session,
}

impl WebSession {
    pub fn new(id: String) -> Self {
        let session = Session::new(id.clone());
        let mut web_session = Self { id, session };

        // Attempt to load existing session data
        if let Err(e) = web_session.load() {
            log::error!("Error loading session: {:?}", e);
            // Optionally, handle the error, e.g., initialize with default session
        }

        web_session
    }

    pub fn load(&mut self) -> Result<(), LoaderError> {
        match LocalStorage::get(&self.id) {
            Ok(Some(stored_session)) => {
                let stored_session: Session = stored_session;
                // Merge the stored session with a newly loaded game
                self.merge_with_new_game(stored_session)?;
                Ok(())
            }
            Ok(None) => {
                log::info!("No session found for ID: {}", self.id);
                Ok(())
            }
            Err(e) => {
                log::error!("Error loading session from storage: {:?}", e);
                Err(LoaderError::StorageError(e))
            }
        }
    }

    pub fn save(&self) -> Result<(), LoaderError> {
        match LocalStorage::set(&self.id, &self.session) {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Error saving session to storage: {:?}", e);
                Err(LoaderError::StorageError(e))
            }
        }
    }

    fn merge_with_new_game(&mut self, stored_session: Session) -> Result<(), LoaderError> {
        let new_game = Game::load_game()?;

        self.session.game_state.game.game_paths = new_game.game_paths;
        self.session.game_state.game.challenge_factory = new_game.challenge_factory;

        self.session.player_profile = stored_session.player_profile;
        self.session.game_state.current_game_path = stored_session.game_state.current_game_path;
        self.session.game_state.current_challenge_index =
            stored_session.game_state.current_challenge_index;
        self.session.game_state.current_task_index = stored_session.game_state.current_task_index;

        self.session
            .game_state
            .game
            .challenge_history
            .merge(&stored_session.game_state.game.challenge_history);

        Ok(())
    }
}

impl Default for WebSession {
    fn default() -> Self {
        match WebSession::load_game() {
            Ok(mut web_session) => {
                if let Err(e) = web_session.load() {
                    log::error!("Error loading session: {:?}", e);
                }
                web_session
            }
            Err(e) => {
                log::error!("Error loading game: {:?}", e);
                WebSession {
                    id: "websession".into(),
                    session: Session::new("websession".into()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_new() {
        let session = WebSession::new("test".into());
        assert_eq!(session.id, "test");
    }
}

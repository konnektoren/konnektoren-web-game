use crate::model::GameLoader;
use gloo::storage::{LocalStorage, Storage};
use konnektoren_core::session::Session;

#[derive(Debug, Clone)]
pub struct WebSession {
    pub id: String,
    pub session: Session,
}

impl WebSession {
    pub fn new(id: String) -> Self {
        let mut session = Self {
            session: Session::new(id.clone()),
            id,
        };
        session.load();
        session
    }

    pub fn load(&mut self) {
        match LocalStorage::get(&self.id) {
            Ok(Some(session)) => {
                let session: Session = session;
                self.session = session;
            }
            Ok(None) => {
                log::info!("No session found");
            }
            Err(e) => {
                log::error!("Error loading session: {:?}", e);
            }
        }
    }

    pub fn save(&self) {
        match LocalStorage::set(&self.id, &self.session) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Error saving session: {:?}", e);
            }
        }
    }
}

impl Default for WebSession {
    fn default() -> Self {
        let mut session = Self::load_game();
        session.load();
        session
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_new() {
        let session = WebSession::new("test".into());
        assert_eq!(session.id, "test");
    }
}

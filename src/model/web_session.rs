use gloo_storage::{LocalStorage, Storage};
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
        let session: Option<Session> = LocalStorage::get(&self.id).unwrap_or_default();
        if let Some(session) = session {
            self.session = session;
        }
    }

    pub fn save(&self) {
        let _ = LocalStorage::set(&self.id, &self.session);
    }
}

impl Default for WebSession {
    fn default() -> Self {
        Self::new("websession".into())
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

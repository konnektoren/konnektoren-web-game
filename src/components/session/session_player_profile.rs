use konnekt_session::model::{Identifiable, Named, PlayerTrait};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct SessionPlayerProfile {
    pub id: String,
    pub name: String,
}

impl Identifiable for SessionPlayerProfile {
    fn identifier(&self) -> &str {
        &self.id
    }
}

impl Named for SessionPlayerProfile {
    fn name(&self) -> &str {
        &self.name
    }
}

impl PlayerTrait for SessionPlayerProfile {}

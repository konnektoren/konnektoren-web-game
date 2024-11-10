use konnekt_session::model::{ActivityData, Identifiable, Named};

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct SessionChallenge {
    pub id: String,
    pub name: String,
}

impl Named for SessionChallenge {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Identifiable for SessionChallenge {
    fn identifier(&self) -> &str {
        &self.id
    }
}

impl ActivityData for SessionChallenge {}

use serde::{Deserialize, Serialize};

use konnekt_session::model::{ActivityResultTrait, Identifiable, Scorable, Timable};

#[derive(PartialEq, Clone, Debug, Hash, Serialize, Deserialize)]
pub struct SessionChallengeResult {
    pub id: String,
    pub performance: u8,
    pub time: u64,
}

impl Identifiable for SessionChallengeResult {
    fn identifier(&self) -> &str {
        &self.id
    }
}

impl Timable for SessionChallengeResult {
    fn time_taken(&self) -> u64 {
        self.time
    }
}

impl Scorable for SessionChallengeResult {
    fn score(&self) -> u32 {
        self.performance as u32
    }
}

impl ActivityResultTrait for SessionChallengeResult {}

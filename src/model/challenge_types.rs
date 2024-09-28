use konnektoren_core::challenges::ChallengeType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ChallengeTypes {
    pub challenges: Vec<ChallengeType>,
}

impl ChallengeTypes {
    pub fn new() -> Self {
        Self { challenges: vec![] }
    }

    pub fn add_challenge(&mut self, challenge: ChallengeType) {
        self.challenges.push(challenge);
    }
}

use konnektoren_core::prelude::{Challenge, ChallengeResult, Performance};
use konnektoren_yew::storage::{ProfileStorage, Storage};

pub fn add_challenge_points(challenge: &Challenge, challenge_result: &ChallengeResult) {
    let performance = challenge.performance(&challenge_result);
    let mut profile = ProfileStorage::default().get("").unwrap_or_default();
    profile.xp += performance / 10;
    ProfileStorage::default().update(profile);
}

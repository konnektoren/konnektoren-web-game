use konnektoren_core::prelude::{Challenge, ChallengeResult, Performance};
use konnektoren_yew::repository::{ProfileRepositoryTrait, PROFILE_STORAGE_KEY};

pub async fn add_challenge_points_to_profile(
    challenge: &Challenge,
    challenge_result: &ChallengeResult,
    repository: &dyn ProfileRepositoryTrait,
) {
    let performance = challenge.performance(challenge_result);
    let mut profile = repository
        .get_profile(PROFILE_STORAGE_KEY)
        .await
        .unwrap()
        .unwrap_or_default();
    profile.xp += performance / 10;
    repository
        .update_profile(PROFILE_STORAGE_KEY, &profile)
        .await
        .unwrap();
}

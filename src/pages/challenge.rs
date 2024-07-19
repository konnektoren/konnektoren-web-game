use crate::components::{ChallengeEffectComponent, ChallengeError, ChallengeFinished};
use crate::model::{ChallengeLoader, WebSession};
use crate::utils::points::add_challenge_points;
use konnektoren_core::prelude::Challenge;
use konnektoren_core::{challenges::ChallengeResult, game::Game};
use konnektoren_yew::components::{MusicComponent, ProfilePointsComponent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengePageProps {
    pub id: String,
}

#[derive(Debug)]
pub enum ChallengeState {
    Challenge(konnektoren_core::challenges::Challenge),
    Finished(
        konnektoren_core::challenges::Challenge,
        konnektoren_core::challenges::ChallengeResult,
    ),
    Error(String),
}

pub fn save_history(challenge: &Challenge, challenge_result: &ChallengeResult) {
    let mut web_session = WebSession::default();
    web_session.load();
    let mut challenge = challenge.clone();
    challenge.challenge_result = challenge_result.clone();
    let session = &mut web_session.session;
    log::info!(
        "Challenge History: {:?}",
        session.game_state.game.challenge_history
    );
    session
        .game_state
        .game
        .challenge_history
        .add_challenge(challenge);
    log::info!(
        "Challenge History: {:?}",
        session.game_state.game.challenge_history
    );
    web_session.save();
}

#[function_component(ChallengePage)]
pub fn challenge_page(props: &ChallengePageProps) -> Html {
    let game = Game::level_a1();

    let challenge_state = use_state(|| match game.create_challenge(&props.id) {
        Ok(challenge) => ChallengeState::Challenge(challenge),
        Err(e) => ChallengeState::Error(e.to_string()),
    });

    let challenge_config = game.game_path.get_challenge_config(&props.id).unwrap();

    match &*challenge_state {
        ChallengeState::Challenge(challenge) => {
            let handle_finish = {
                let challenge_state = challenge_state.clone();
                let challenge = challenge.clone();
                Callback::from(move |result: ChallengeResult| {
                    let challenge = challenge.clone();
                    log::info!("Challenge Result: {:?}", result);
                    add_challenge_points(&challenge, &result);
                    save_history(&challenge, &result);
                    challenge_state.set(ChallengeState::Finished(challenge, result))
                })
            };

            html! {
                <div class="challenge-page">
                    <MusicComponent url="/music/background_main.wav" />
                    <ProfilePointsComponent />
                    <ChallengeEffectComponent challenge={challenge.clone()} variant={challenge_config.variant.clone()} on_finish={handle_finish} />
                </div>
            }
        }
        ChallengeState::Finished(challenge, challenge_result) => {
            html! {
                <div class="challenge-page">
                    <MusicComponent url="/music/background_main.wav" />
                    <ChallengeFinished challenge={challenge.clone()} challenge_result={challenge_result.clone()} />
                </div>
            }
        }
        ChallengeState::Error(err) => {
            html!(
                <div class="challenge-page">
                    <ChallengeError error={err.clone()} />
                </div>
            )
        }
    }
}

use crate::components::{ChallengeEffectComponent, ChallengeError, ChallengeFinished};
use crate::model::{ChallengeLoader, WebSession};
use crate::route::Route;
use crate::utils::points::add_challenge_points;
use gloo::utils::document;
use konnektoren_core::challenges::{ChallengeHistory, PerformanceRecord};
use konnektoren_core::prelude::Challenge;
use konnektoren_core::{challenges::ChallengeResult, game::Game};
use konnektoren_yew::components::{MusicComponent, ProfilePointsComponent};
use konnektoren_yew::storage::{ProfileStorage, Storage};
use reqwest::Client;
use yew::prelude::*;
use yew_router::prelude::Link;

const API_URL: &str = "https://api.konnektoren.help/api/v1/performance-record";

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
    Results(
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
    let challenge_config = game.game_path.get_challenge_config(&props.id).unwrap();
    let challenge_name = challenge_config.name.clone();
    use_effect(move || {
        document().set_title(&format!("Konnektoren - {}", challenge_name));
        || ()
    });

    let challenge_state = use_state(|| match game.create_challenge(&props.id) {
        Ok(challenge) => ChallengeState::Challenge(challenge),
        Err(e) => ChallengeState::Error(e.to_string()),
    });

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
                    <Link<Route> to={Route::Profile}><ProfilePointsComponent /></Link<Route>>
                    <ChallengeEffectComponent challenge={challenge.clone()} variant={challenge_config.variant.clone()} on_finish={handle_finish} />
                </div>
            }
        }
        ChallengeState::Finished(challenge, challenge_result) => {
            let challenge = challenge.clone();
            let challenge_result = challenge_result.clone();

            let web_session = WebSession::default();
            let profile = ProfileStorage::default().get("").unwrap_or_default();

            let profile_name = profile.name.clone();
            let game_path_id = web_session.session.game_state.game.game_path.id.clone();

            let challenge_id = &challenge.challenge_config.id.clone();
            let url = format!("{}/{}", API_URL, challenge_id);

            let mut challenge = challenge.clone();
            challenge.challenge_result = challenge_result.clone();
            let mut challenge_history: ChallengeHistory = ChallengeHistory::new();
            challenge_history.add_challenge(challenge.clone());

            let performance_record = PerformanceRecord::new_from_history(
                game_path_id.clone(),
                profile_name.clone(),
                1,
                challenge_history,
            );

            wasm_bindgen_futures::spawn_local(async move {
                let client = Client::new();
                let response = client.post(url).json(&performance_record).send().await;

                if response.is_err() {
                    let err = response.err().unwrap();
                    log::error!("Error sending performance record: {:?}", err);
                }
                challenge_state.set(ChallengeState::Results(
                    challenge.clone(),
                    challenge_result.clone(),
                ));
            });

            html! {}
        }
        ChallengeState::Results(challenge, challenge_result) => {
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

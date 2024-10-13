use crate::components::{ChallengeEffectComponent, ChallengeError, ChallengeFinished};
use crate::model::{GameLoader, WebSession};
use crate::utils::points::add_challenge_points;
use crate::Route;
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
    let mut web_session = match WebSession::load_game() {
        Ok(ws) => ws,
        Err(e) => {
            log::error!("Error loading web session: {:?}", e);
            return;
        }
    };

    web_session.load().unwrap_or_default();

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
    if let Err(e) = web_session.save() {
        log::error!("Error saving web session: {:?}", e);
    }
}

#[function_component(ChallengePage)]
pub fn challenge_page(props: &ChallengePageProps) -> Html {
    // Load the game and handle any errors
    let game_result = Game::load_game();
    let game = match game_result {
        Ok(game) => game,
        Err(e) => {
            return html! {
                <div class="challenge-page">
                    <ChallengeError error={format!("Error loading game: {:?}", e)} />
                </div>
            };
        }
    };

    // Load the web session and handle any errors
    let mut web_session = match WebSession::load_game() {
        Ok(ws) => ws,
        Err(e) => {
            return html! {
                <div class="challenge-page">
                    <ChallengeError error={format!("Error loading session: {:?}", e)} />
                </div>
            };
        }
    };

    web_session.load().unwrap_or_default();

    let current_level = web_session.session.game_state.current_game_path;

    // Safely get the current game path
    let game_path = match game.game_paths.get(current_level) {
        Some(path) => path,
        None => {
            return html! {
                <div class="challenge-page">
                    <ChallengeError error={format!("Invalid current level: {}", current_level)} />
                </div>
            };
        }
    };

    // Safely get the challenge configuration
    let challenge_config = match game_path.get_challenge_config(&props.id) {
        Some(config) => config.clone(),
        None => {
            return html! {
                <div class="challenge-page">
                    <ChallengeError error={"Challenge not found".to_string()} />
                </div>
            };
        }
    };

    let challenge_name = challenge_config.name.clone();
    use_effect(move || {
        document().set_title(&format!("Konnektoren - {}", challenge_name));
        || ()
    });

    // Initialize the challenge state with error handling
    let challenge_state = use_state(|| match game.create_challenge(&props.id) {
        Ok(challenge) => ChallengeState::Challenge(challenge),
        Err(e) => ChallengeState::Error(e.to_string()),
    });

    {
        let id = props.id.clone();
        let challenge_state = challenge_state.clone();
        let game = game.clone();

        use_effect_with(id.clone(), move |id| {
            challenge_state.set(match game.create_challenge(id) {
                Ok(challenge) => ChallengeState::Challenge(challenge),
                Err(e) => ChallengeState::Error(e.to_string()),
            });
            || ()
        });
    }

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

            // Load the web session and handle any errors
            let web_session = match WebSession::load_game() {
                Ok(ws) => ws,
                Err(e) => {
                    return html! {
                        <div class="challenge-page">
                            <ChallengeError error={format!("Error loading session: {:?}", e)} />
                        </div>
                    };
                }
            };

            let profile = ProfileStorage::default().get("").unwrap_or_default();
            let profile_name = profile.name.clone();

            let current_level = web_session.session.game_state.current_game_path;
            let game_path_id = match web_session
                .session
                .game_state
                .game
                .game_paths
                .get(current_level)
            {
                Some(game_path) => game_path.id.clone(),
                None => {
                    return html! {
                        <div class="challenge-page">
                            <ChallengeError error={format!("Invalid current level: {}", current_level)} />
                        </div>
                    };
                }
            };

            let challenge_id = challenge.challenge_config.id.clone();
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

            let challenge_state_clone = challenge_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let client = Client::new();
                match client.post(&url).json(&performance_record).send().await {
                    Ok(response) => {
                        if !response.status().is_success() {
                            log::error!(
                                "Error sending performance record: {:?}",
                                response.status()
                            );
                        }
                    }
                    Err(e) => {
                        log::error!("Error sending performance record: {:?}", e);
                    }
                }
                challenge_state_clone.set(ChallengeState::Results(
                    challenge.clone(),
                    challenge_result.clone(),
                ));
            });

            html! {}
        }
        ChallengeState::Results(challenge, challenge_result) => {
            let next_challenge = game_path.next_challenge_id(&props.id);
            html! {
                <div class="challenge-page">
                    <MusicComponent url="/music/background_main.wav" />
                    <ChallengeFinished challenge={challenge.clone()} challenge_result={challenge_result.clone()}
                        next_challenge={next_challenge} />
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

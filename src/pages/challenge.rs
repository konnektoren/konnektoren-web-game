use crate::components::{ChallengeEffectComponent, ChallengeError, ChallengeFinished};

use crate::utils::seo;
use crate::Route;
use konnektoren_core::challenges::{ChallengeHistory, Performance, PerformanceRecord};
use konnektoren_core::challenges::{ChallengeResult, Timed};
use konnektoren_yew::components::MusicComponent;
use konnektoren_yew::i18n::use_selected_language;
use konnektoren_yew::managers::ProfilePointsManager;
use konnektoren_yew::prelude::{use_game_state, use_profile};
use reqwest::Client;
use yew::prelude::*;
use yew_router::prelude::Link;

const API_URL: &str = "https://api.konnektoren.help/api/v1/performance-record";
const SITE_URL: &str = "https://konnektoren.app";
const IMAGE_URL: &str = "https://konnektoren.app/favicon.png";
const KEY_WORDS: &str = "German Grammar,Artikel, Personalpronoment, der-die-das, Reflexivpronomen, Konnektoren, Learning Platform, Gamification, Verifiable Credential";

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

#[function_component(ChallengePage)]
pub fn challenge_page(props: &ChallengePageProps) -> Html {
    let profile = use_profile();
    let language = use_selected_language();
    let game_state = use_game_state();
    let game = game_state.game.clone();

    let current_level = game_state.current_game_path;
    log::info!("Current level: {}", current_level);

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
                    <ChallengeError error={format!("Challenge {} not found in Level {}, available: {:?}", props.id, current_level, game_path.challenge_ids())} />
                </div>
            };
        }
    };

    let challenge_name = challenge_config.name.clone();
    let challenge_description = challenge_config.description.clone();
    let lang = language.get();
    use_effect(move || {
        seo::update_meta_tags(
            &challenge_name,
            &challenge_description,
            KEY_WORDS,
            SITE_URL,
            IMAGE_URL,
            &lang,
        );

        || ()
    });

    // Initialize the challenge state with error handling
    let challenge_state = use_state(|| match game.create_challenge(&props.id) {
        Ok(challenge) => {
            let mut challenge = challenge;
            challenge.start();
            ChallengeState::Challenge(challenge)
        }
        Err(e) => ChallengeState::Error(e.to_string()),
    });

    {
        let id = props.id.clone();
        let challenge_state = challenge_state.clone();
        let game = game.clone();

        use_effect_with(id.clone(), move |id| {
            challenge_state.set(match game.create_challenge(id) {
                Ok(challenge) => {
                    let mut challenge = challenge.clone();
                    challenge.start();
                    ChallengeState::Challenge(challenge)
                }
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
                let profile = profile.clone();
                Callback::from(move |result: ChallengeResult| {
                    let result = result.clone();
                    let mut challenge = challenge.clone();
                    challenge.update_end_time();
                    challenge_state
                        .set(ChallengeState::Finished(challenge.clone(), result.clone()));
                    let performance = challenge.performance(&result);
                    let mut new_profile = (&*profile).clone();
                    new_profile.xp += performance / 10;

                    profile.set(new_profile);
                })
            };

            html! {
                <div class="challenge-page">
                    <MusicComponent url="/music/background_main.wav" />
                    <Link<Route> to={Route::Profile}>
                        <ProfilePointsManager/>
                    </Link<Route>>
                    <ChallengeEffectComponent challenge={challenge.clone()} variant={challenge_config.variant.clone()} on_finish={handle_finish} />
                </div>
            }
        }
        ChallengeState::Finished(challenge, challenge_result) => {
            let challenge = challenge.clone();
            let challenge_result = challenge_result.clone();

            let profile_name = profile.name.clone();

            let current_level = game_state.current_game_path;
            let game_path_id = match game_state.game.game_paths.get(current_level) {
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

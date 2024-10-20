use crate::components::ChallengeCard;
use crate::Route;
use konnektoren_yew::components::{MusicComponent, SelectLevelComp};
use konnektoren_yew::managers::ProfilePointsManager;
use konnektoren_yew::prelude::{use_i18n, use_profile, use_session, use_session_repository};
use konnektoren_yew::repository::SESSION_STORAGE_KEY;
use yew::prelude::*;
use yew_router::components::Link;

const API_URL: &str = "https://api.konnektoren.help/api/v1/reviews";

#[function_component(ChallengesPage)]
pub fn challenges_page() -> Html {
    let i18n = use_i18n();
    let profile = use_profile().read().unwrap().clone();
    let session = use_session();
    let session_repositoy = use_session_repository();

    let game_state = session.read().unwrap().game_state.clone();
    let challenge_history = game_state.game.challenge_history.clone();

    let game_paths = game_state.game.game_paths.clone();
    let current_level = use_state(|| session.read().unwrap().game_state.current_game_path);

    // Callback for switching levels
    let switch_level = {
        let session = session.clone();
        let session_repositoy = session_repositoy.clone();
        let current_level = current_level.clone();
        Callback::from(move |level: usize| {
            let session = session.clone();
            let session_repositoy = session_repositoy.clone();
            let mut new_session = session.read().unwrap().clone();
            new_session.game_state.current_game_path = level;

            let mut session_guard = session.write().unwrap();
            *session_guard = new_session.clone();

            current_level.set(level);
            wasm_bindgen_futures::spawn_local(async move {
                session_repositoy
                    .save_session(SESSION_STORAGE_KEY, &new_session)
                    .await
                    .unwrap();
            });
        })
    };

    // Generate the challenges list
    let challenges_list = game_paths[*current_level]
        .challenges
        .iter()
        .enumerate()
        .map(|(index, config)| {
            if config.unlock_points as u32 > profile.xp {
                html! {
                    <div class="challenge" key={index}>
                        <ChallengeCard challenge_config={config.clone()} />
                    </div>
                }
            } else {
                html! {
                    <div class="challenge" key={index}>
                        <ChallengeCard api_url={Some(API_URL)} challenge_config={config.clone()}
                            challenge_history={challenge_history.clone()} />
                    </div>
                }
            }
        })
        .collect::<Html>();

    html! {
        <div class="challenges-page">
            <MusicComponent url="music/background_main.wav" />
            <Link<Route> to={Route::Profile}>
                <ProfilePointsManager/>
                </Link<Route>>
            <h1>{ i18n.t("Challenges") }</h1>
            <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={switch_level} />
            { challenges_list }
        </div>
    }
}

use crate::components::ChallengeCard;
use crate::model::{LoaderError, WebSession};
use crate::route::Route;
use konnektoren_yew::components::{MusicComponent, ProfilePointsComponent, SelectLevelComp};
use konnektoren_yew::prelude::use_i18n;
use konnektoren_yew::storage::{ProfileStorage, Storage};
use yew::prelude::*;
use yew_router::components::Link;

const API_URL: &str = "https://api.konnektoren.help/api/v1/reviews";

#[function_component(ChallengesPage)]
pub fn challenges_page() -> Html {
    let i18n = use_i18n();
    let profile = ProfileStorage::default().get("").unwrap_or_default();

    let load_error = use_state(|| Option::<LoaderError>::None);

    let web_session = use_state(|| WebSession::default());

    {
        let load_error = load_error.clone();
        let web_session = web_session.clone();
        use_effect_with((), move |_| {
            let mut session = (*web_session).clone();
            match session.load() {
                Ok(_) => {
                    web_session.set(session);
                }
                Err(LoaderError::StorageError(e)) => {
                    log::info!("Session not found: {}, using default session", e);
                    web_session.set(WebSession::default());
                }
                Err(e) => {
                    load_error.set(Some(e));
                }
            }
            || ()
        });
    }

    // Early return if there is a loading error
    if let Some(error) = &*load_error {
        return html! {
            <div class="error-message">
                { format!("Error loading session: {:?}", error) }
            </div>
        };
    }

    let game_paths = web_session.session.game_state.game.game_paths.clone();
    let current_level = use_state(|| web_session.session.game_state.current_game_path);

    // Callback for switching levels
    let switch_level = {
        let web_session = web_session.clone();
        let current_level = current_level.clone();
        let load_error = load_error.clone();
        Callback::from(move |level: usize| {
            let mut web_session = (*web_session).clone();
            web_session.session.game_state.current_game_path = level;

            if let Err(e) = web_session.save() {
                load_error.set(Some(e));
            } else {
                current_level.set(level);
            }
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
                        <ChallengeCard api_url={Some(API_URL)} challenge_config={config.clone()} />
                    </div>
                }
            }
        })
        .collect::<Html>();

    html! {
        <div class="challenges-page">
            <MusicComponent url="music/background_main.wav" />
            <Link<Route> to={Route::Profile}><ProfilePointsComponent /></Link<Route>>
            <h1>{ i18n.t("Challenges") }</h1>
            <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={switch_level} />
            { challenges_list }
        </div>
    }
}

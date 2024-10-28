use crate::components::ChallengeCard;
use crate::Route;
use konnektoren_yew::components::{MusicComponent, SelectLevelComp};
use konnektoren_yew::managers::ProfilePointsManager;
use konnektoren_yew::prelude::{use_game_state, use_i18n, use_profile, use_session};
use yew::prelude::*;
use yew_router::components::Link;

const API_URL: &str = "https://api.konnektoren.help/api/v1/reviews";

#[function_component(ChallengesPage)]
pub fn challenges_page() -> Html {
    let i18n = use_i18n();
    let profile = use_profile();
    let session = use_session();
    let game_state = use_game_state();

    let game = game_state.lock().unwrap().game.clone();

    let challenge_history = game.challenge_history.clone();

    let game_paths = game.game_paths.clone();
    let current_level = use_state(|| session.game_state.current_game_path);

    let handle_switch_level = {
        let session = session.clone();
        let current_level = current_level.clone();
        Callback::from(move |level: usize| {
            let session = session.clone();
            let mut new_session = (&*session).clone();
            new_session.game_state.current_game_path = level;
            session.set(new_session);
            current_level.set(level);
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
            <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={handle_switch_level} />
            { challenges_list }
        </div>
    }
}

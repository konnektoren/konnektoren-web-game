use crate::components::map::ChallengeInfo;
use crate::model::WebSession;
use crate::route::Route;
use konnektoren_yew::components::{MusicComponent, ProfilePointsComponent, SelectLevelComp};
use konnektoren_yew::storage::{ProfileStorage, Storage};
use yew::prelude::*;
use yew_router::components::Link;

#[function_component(ChallengesPage)]
pub fn challenges_page() -> Html {
    let profile = ProfileStorage::default().get("").unwrap_or_default();
    let mut web_session = WebSession::default();
    web_session.load();

    let game_paths = web_session.session.game_state.game.game_paths.clone();
    let current_level = use_state(|| web_session.session.game_state.current_game_path);

    let switch_level = {
        let web_session = web_session.clone();
        let current_level = current_level.clone();
        Callback::from(move |level: usize| {
            let mut web_session = web_session.clone();
            web_session.session.game_state.current_game_path = level;
            web_session.save();
            current_level.set(level);
        })
    };

    let challenges_list = game_paths[*current_level]
        .challenges
        .iter()
        .enumerate()
        .map(|(index, config)| {
            if config.unlock_points as u32 > profile.xp {
                return html! {
                    <div class="challenge" key={index}>
                        <ChallengeInfo challenge_config={config.clone()} />
                    </div>
                };
            }
            html! {
                <div class="challenge" key={index}>
                    <ChallengeInfo challenge_config={config.clone()} />
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="challenges-page">
            <MusicComponent url="music/background_main.wav" />
            <Link<Route> to={Route::Profile}><ProfilePointsComponent /></Link<Route>>
            <h1>{ "Challenges" }</h1>
            <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={switch_level} />
            { challenges_list }
        </div>
    }
}

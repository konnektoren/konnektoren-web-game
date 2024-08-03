use konnektoren_yew::components::game_map::{ChallengeIndex, Coordinate, GameMapComponent};
use konnektoren_yew::storage::{ProfileStorage, Storage};
use yew::{callback, prelude::*};

use crate::model::{ChallengeLoader, WebSession};

mod challenge_info;
mod challenge_navigation;

pub use challenge_info::ChallengeInfo;
pub use challenge_navigation::ChallengeNavigationComp;

#[function_component(Map)]
pub fn map() -> Html {
    let profile = ProfileStorage::default().get("").unwrap_or_default();
    let mut web_session = WebSession::level_a1();
    web_session.load();

    let game_path = web_session.session.game_state.game.game_path.clone();
    let current_challenge = use_state(|| web_session.session.game_state.current_challenge_index);
    let challenge_info_position = use_state(Coordinate::default);

    let current_challenge_clone = current_challenge.clone();
    let challenge_info_position_clone = challenge_info_position.clone();
    let web_session_clone = web_session.clone();
    let callback = callback::Callback::from(
        move |(challenge_index, (x, y)): (Option<ChallengeIndex>, Coordinate)| {
            if let Some(challenge_index) = challenge_index {
                let mut web_session = web_session_clone.clone();
                current_challenge_clone.set(challenge_index);
                challenge_info_position_clone.set((x, y));
                web_session.session.game_state.current_challenge_index = challenge_index;
                web_session.save();
                log::info!("Challenge selected: {}", challenge_index);
            } else {
                challenge_info_position_clone.set((0, 0));
                log::info!("Challenge deselected {} {}", x, y);
            }
        },
    );

    let challenge_config = web_session
        .session
        .game_state
        .game
        .game_path
        .challenges
        .get(*current_challenge);

    let points = profile.xp;

    let (x, y) = *challenge_info_position;
    let (x, y) = (x.max(0), y.max(0));

    html! {
        <div class="map-container" id={format!("{}", *current_challenge)}>
            {
                if let Some(config) = challenge_config {
                    if x > 0 && y > 0 {
                    html! {
                        <div style={format!("position: absolute; top: {}px; left: {}px;", y, x)}>
                            <ChallengeInfo challenge_config={config.clone()} />
                        </div>
                    }
                    } else {
                        html! {}
                    }
                } else {
                    html! {}
                }
            }
            <div class="map-svg-items">
                <div id="small_ship_1" />
                <div id="small_ship_2" />
                <div id="huge_ship" />
                <div id="plane" />
                <div id="clouds" />
                <div id="huge_waves" />
                <div id="small_waves" />
            </div>
            <GameMapComponent game_path={game_path.clone()} current_challenge={*current_challenge}
                on_select_challenge={Some(callback.clone())} points={points as usize} />
            <ChallengeNavigationComp game_path={game_path} current_challenge={*current_challenge}
                on_select_challenge={Some(callback)} />
        </div>
    }
}

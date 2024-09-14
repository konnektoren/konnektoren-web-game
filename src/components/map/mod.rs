use konnektoren_yew::components::MapComponent;
use konnektoren_yew::prelude::{BrowserCoordinate, ChallengeIndex, SelectLevelComp};
use konnektoren_yew::storage::{ProfileStorage, Storage};
use yew::{callback, prelude::*};

use crate::model::WebSession;

mod challenge_info;
mod challenge_navigation;
mod info_position;

pub use challenge_info::ChallengeInfo;
pub use challenge_navigation::ChallengeNavigationComp;

#[function_component(Map)]
pub fn map() -> Html {
    let profile = ProfileStorage::default().get("").unwrap_or_default();
    let mut web_session = WebSession::default();
    web_session.load();

    let arrow_visible = use_state(|| true);
    {
        let arrow_visible = arrow_visible.clone();
        use_effect(move || {
            let timeout = gloo::timers::callback::Timeout::new(16000, move || {
                arrow_visible.set(false);
            });
            || drop(timeout)
        });
    }

    let game_paths = web_session.session.game_state.game.game_paths.clone();
    let current_level = use_state(|| web_session.session.game_state.current_game_path);
    let current_challenge = use_state(|| web_session.session.game_state.current_challenge_index);
    let challenge_info_position = use_state(BrowserCoordinate::default);

    let current_challenge_clone = current_challenge.clone();
    let challenge_info_position_clone = challenge_info_position.clone();
    let web_session_clone = web_session.clone();
    let callback = callback::Callback::from(
        move |(challenge_index, coord): (Option<ChallengeIndex>, BrowserCoordinate)| {
            if let Some(challenge_index) = challenge_index {
                let mut web_session = web_session_clone.clone();
                current_challenge_clone.set(challenge_index);
                challenge_info_position_clone.set(coord);
                web_session.session.game_state.current_challenge_index = challenge_index;
                web_session.save();
                log::info!("Challenge selected: {}", challenge_index);
            } else {
                challenge_info_position_clone.set(BrowserCoordinate::default());
                log::info!("Challenge deselected {} {}", coord.0, coord.1);
            }
        },
    );

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

    let challenge_config = web_session.session.game_state.game.game_paths[*current_level]
        .challenges
        .get(*current_challenge);

    let points = profile.xp;

    let x = challenge_info_position.0;
    let y = challenge_info_position.1;
    let (x, y) = (x.max(0.0), y.max(0.0));

    let dialog_width = 350.0;
    let dialog_height = 200.0;

    let (adjusted_x, adjusted_y) =
        info_position::adjust_info_position(x, y, dialog_width, dialog_height);

    html! {
        <div class="map-container" id={format!("{}", *current_challenge)}>
            {
                if let Some(config) = challenge_config {
                    if x > 0.0 && y > 0.0 {
                    html! {
                        <div style={format!("position: absolute; top: {}px; left: {}px;", adjusted_y, adjusted_x)}>
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
            {
                match *arrow_visible {
                    true => html! {
                        <div id="Arrow_1_Red" class="visible"></div>
                    },
                    false => html! {}
                }
            }
            <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={switch_level} />
            <MapComponent game_path={game_paths[*current_level].clone()} current_challenge={*current_challenge}
                on_select_challenge={Some(callback.clone())} points={points as usize}
                image_src={"/assets/images/German_Map_Animated.svg"}/>
            <ChallengeNavigationComp game_path={game_paths[*current_level].clone()} current_challenge={*current_challenge}
                on_select_challenge={Some(callback)} />
        </div>
    }
}

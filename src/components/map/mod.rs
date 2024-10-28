use konnektoren_yew::components::MapComponent;
use konnektoren_yew::prelude::{
    use_profile, use_session, BrowserCoordinate, ChallengeIndex, SelectLevelComp,
};
use yew::prelude::*;

use crate::model::LoaderError;

mod challenge_info;
mod challenge_navigation;
mod info_position;

pub use challenge_info::ChallengeInfo;
pub use challenge_navigation::ChallengeNavigationComp;

#[function_component(Map)]
pub fn map() -> Html {
    let profile = use_profile();
    let session = use_session();

    let load_error = use_state(|| Option::<LoaderError>::None);

    if let Some(error) = &*load_error {
        return html! {
            <div class="error-message">
                { format!("Error loading game or session: {:?}", error) }
            </div>
        };
    }

    let game_state = session.game_state.clone();

    let game_paths = game_state.game.game_paths.clone();
    let max_level = game_paths.len();

    let current_level_value = game_state.current_game_path;
    let current_level = use_state(move || {
        if current_level_value < max_level {
            current_level_value
        } else {
            0
        }
    });

    let current_game_path = match game_paths.get(*current_level) {
        Some(game_path) => game_path.clone(),
        None => {
            return html! {
                <div class="error-message">
                    { "Error: Invalid level selected." }
                </div>
            };
        }
    };

    let max_challenge = current_game_path.challenges.len();
    let current_challenge_value = game_state.current_challenge_index;
    let current_challenge = use_state(move || {
        if current_challenge_value < max_challenge {
            current_challenge_value
        } else {
            0
        }
    });

    let challenge_info_position = use_state(BrowserCoordinate::default);

    let current_challenge_clone = current_challenge.clone();
    let challenge_info_position_clone = challenge_info_position.clone();

    let callback = {
        let session = session.clone();
        Callback::from(
            move |(challenge_index, coord): (Option<ChallengeIndex>, BrowserCoordinate)| {
                let session = session.clone();
                if let Some(challenge_index) = challenge_index {
                    let mut new_session = (&*session).clone();
                    current_challenge_clone.set(challenge_index);
                    challenge_info_position_clone.set(coord);
                    new_session.game_state.current_challenge_index = challenge_index;
                    session.set(new_session);
                    log::info!("Challenge selected: {}", challenge_index);
                } else {
                    challenge_info_position_clone.set(BrowserCoordinate::default());
                    log::info!("Challenge deselected {} {}", coord.0, coord.1);
                }
            },
        )
    };

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

    let challenge_config = current_game_path.challenges.get(*current_challenge);

    let points = profile.xp;

    let x = challenge_info_position.0;
    let y = challenge_info_position.1;
    let (x, y) = (x.max(0.0), y.max(0.0));

    let dialog_width = 350.0;
    let dialog_height = 200.0;

    let (adjusted_x, adjusted_y) =
        info_position::adjust_info_position(x, y, dialog_width, dialog_height);

    // Render the component
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
                    html! {
                        <div class="error-message">
                            { "Error: Challenge configuration not found." }
                        </div>
                    }
                }
            }
            <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={handle_switch_level} />
            <MapComponent game_path={current_game_path.clone()} current_challenge={*current_challenge}
                on_select_challenge={Some(callback.clone())} points={points as usize}
                image_src={"/assets/images/German_Map_Animated.svg"}/>
            <ChallengeNavigationComp game_path={current_game_path.clone()} current_challenge={*current_challenge}
                on_select_challenge={Some(callback)} />
        </div>
    }
}

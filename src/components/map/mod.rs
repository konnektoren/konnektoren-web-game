use konnektoren_yew::components::game_map::GameMapComponent;
use yew::{callback, prelude::*};

use crate::model::WebSession;

mod challenge_info;
pub use challenge_info::ChallengeInfo;

#[function_component(Map)]
pub fn map() -> Html {
    let web_session = WebSession::default();

    let game_path = web_session.session.game_state.game.game_path.clone();
    let current_challenge = use_state(|| web_session.session.game_state.current_challenge_index);

    let current_challenge_clone = current_challenge.clone();
    let callback = callback::Callback::from(move |challenge_index| {
        current_challenge_clone.set(challenge_index);
        let mut web_session = WebSession::default();
        web_session.session.game_state.current_challenge_index = challenge_index;
        web_session.save();
        log::info!("Challenge selected: {}", challenge_index);
    });

    let challenge_config = web_session
        .session
        .game_state
        .game
        .game_path
        .challenges
        .get(*current_challenge);

    html! {
        <div class="map" id={format!("{}", *current_challenge)}>
            {
                if let Some(config) = challenge_config {
                    html! {
                        <ChallengeInfo challenge_config={config.clone()} />
                    }
                } else {
                    html! {}
                }
            }
            <GameMapComponent {game_path} current_challenge={*current_challenge} on_select_challenge={Some(callback)} />
        </div>
    }
}

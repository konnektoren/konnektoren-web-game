use konnektoren_core::game::GamePath;
use konnektoren_yew::components::game_map::GameMapComponent;
use yew::prelude::*;

#[function_component(Map)]
pub fn map() -> Html {
    let game_path = GamePath::default();
    let current_challenge = 1;
    html! {
        <div class="map">
            <GameMapComponent {game_path} {current_challenge} />
        </div>
    }
}

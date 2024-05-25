use yew::prelude::*;

use konnektoren_core::challenges::ChallengeConfig;
use yew_router::hooks::use_navigator;

use crate::route::Route;

#[derive(Properties, PartialEq)]
pub struct ChallengeInfoProps {
    pub challenge_config: ChallengeConfig,
}

#[function_component(ChallengeInfo)]
pub fn challenge_info(props: &ChallengeInfoProps) -> Html {
    let id = props.challenge_config.id.clone();
    let title = props.challenge_config.name.clone();
    let description = props.challenge_config.description.clone();

    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        let id = id.clone();
        log::info!("Challenge selected: {}", id);
        navigator.push(&Route::Challenge { id });
    });

    html! {
        <div class="challenge-info">
            <h1>{title}</h1>
            <p>{description}</p>
            <button onclick={onclick}>{ "Start" }</button>
        </div>
    }
}

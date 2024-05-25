use yew::prelude::*;

use konnektoren_core::challenges::ChallengeConfig;
use konnektoren_yew::components::challenge_config::ChallengeConfigComponent;
use yew_router::hooks::use_navigator;

use crate::route::Route;

#[derive(Properties, PartialEq)]
pub struct ChallengeInfoProps {
    pub challenge_config: ChallengeConfig,
}

#[function_component(ChallengeInfo)]
pub fn challenge_info(props: &ChallengeInfoProps) -> Html {
    let id = props.challenge_config.id.clone();
    let navigator = use_navigator().unwrap();
    let on_new = Callback::from(move |_| {
        let id = id.clone();
        log::info!("Challenge selected: {}", id);
        navigator.push(&Route::Challenge { id });
    });

    html! {
        <ChallengeConfigComponent on_new={on_new} challenge_config={props.challenge_config.clone()} />
    }
}

use crate::Route;
use konnektoren_core::{challenges::ChallengeConfig, prelude::PlayerProfile};
use konnektoren_yew::{
    components::challenge_config::ChallengeConfigComponent, prelude::use_profile,
};
use web_sys::window;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Properties, PartialEq)]
pub struct ChallengeInfoProps {
    pub challenge_config: ChallengeConfig,
    #[prop_or_default]
    pub api_url: Option<String>,
}

pub fn challenge_unlocked(challenge_config: &ChallengeConfig, profile: &PlayerProfile) -> bool {
    profile.xp >= challenge_config.unlock_points as u32
}

#[function_component(ChallengeInfo)]
pub fn challenge_info(props: &ChallengeInfoProps) -> Html {
    let id = props.challenge_config.id.clone();
    let navigator = use_navigator().unwrap();
    let profile = use_profile().read().unwrap().clone();
    let on_new = {
        if challenge_unlocked(&props.challenge_config, &profile) {
            Callback::from(move |_| {
                let id = id.clone();
                log::info!("Challenge selected: {}", id);
                navigator.push(&Route::Challenge { id });
            })
        } else {
            Callback::from(move |_| {
                log::info!("Challenge selected: {}", id);
                window()
                    .unwrap()
                    .alert_with_message("Challenge is locked")
                    .expect("alert failed");
            })
        }
    };

    html! {
        <ChallengeConfigComponent api_url={props.api_url.clone()} on_new={on_new} challenge_config={props.challenge_config.clone()} />
    }
}

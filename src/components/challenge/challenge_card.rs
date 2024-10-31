use crate::components::map::ChallengeInfo;
use crate::components::ChallengePreviewComponent;
use konnektoren_core::challenges::{ChallengeConfig, ChallengeHistory, ChallengeStats};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct ChallengeCardProps {
    pub challenge_config: ChallengeConfig,
    #[prop_or_default]
    pub challenge_history: ChallengeHistory,
    #[prop_or_default]
    pub api_url: Option<String>,
}

#[function_component(ChallengeCard)]
pub fn challenge_card(props: &ChallengeCardProps) -> Html {
    let flipped = use_state(|| false);

    let on_flip = {
        let flipped = flipped.clone();
        Callback::from(move |_| {
            flipped.set(!*flipped);
        })
    };

    let flip_class = if *flipped {
        "challenge-inner challenge-flipped"
    } else {
        "challenge-inner"
    };

    let challenge_config = props.challenge_config.clone();
    let api_url = props.api_url.clone();

    let challenge_preview_page = match *flipped {
        true => html! {<ChallengePreviewComponent challenge_config={challenge_config.clone()} />},
        false => html! {},
    };

    let solved_icon = match (&props.challenge_config, &props.challenge_history).solved() {
        true => html! {<div class="solved-icon"><i class="fas fa-check-circle"></i></div>},
        false => html! {},
    };

    html! {
        <div class="challenge-card" onclick={on_flip}>
            {solved_icon}
            <div class={flip_class}>
                <div class="challenge-front">
                    <ChallengeInfo {api_url} {challenge_config} />
                </div>
                <div class="challenge-back">
                    <div class="challenge-preview-container">
                        {challenge_preview_page}
                    </div>
                </div>
            </div>
        <div class="flip-hint">{"Click to Flip"}</div>
        </div>
    }
}

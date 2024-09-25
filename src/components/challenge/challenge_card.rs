use crate::components::map::ChallengeInfo;
use konnektoren_core::challenges::ChallengeConfig;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ChallengeCardProps {
    pub challenge_config: ChallengeConfig,
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
    html! {
        <div class="challenge-card" onclick={on_flip}>
            <div class={flip_class}>
                <div class="challenge-front">
                    <ChallengeInfo {api_url} {challenge_config} />
                </div>
                <div class="challenge-back">
                    <p>{ "Preview of the exercise" }</p>
                </div>
            </div>
        </div>
    }
}

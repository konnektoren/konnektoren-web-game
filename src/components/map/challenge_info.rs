use yew::prelude::*;

use konnektoren_core::challenges::ChallengeConfig;

#[derive(Properties, PartialEq)]
pub struct ChallengeInfoProps {
    pub challenge_config: ChallengeConfig,
}

#[function_component(ChallengeInfo)]
pub fn challenge_info(props: &ChallengeInfoProps) -> Html {
    let title = props.challenge_config.name.clone();
    let description = props.challenge_config.description.clone();

    html! {
        <div class="challenge-info">
            <h1>{title}</h1>
            <p>{description}</p>
        </div>
    }
}

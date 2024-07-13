use crate::i18n;
use crate::route::Route;
use konnektoren_core::challenges::ChallengeType;
use konnektoren_core::prelude::{Challenge, ChallengeResult};
use konnektoren_yew::components::challenge::{
    MultipleChoiceResultComponent, ResultSummaryComponent,
};
use konnektoren_yew::effects::BlinkAnimation;
use std::time::Duration;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub challenge: Challenge,
    pub challenge_result: ChallengeResult,
}

#[function_component(ChallengeFinished)]
pub fn challenge_finished(props: &Props) -> Html {
    let Props {
        challenge,
        challenge_result,
    } = props;

    let challenge_result_component = match &challenge.challenge_type {
        ChallengeType::MultipleChoice(challenge) => html! {
            <MultipleChoiceResultComponent challenge={challenge.clone()} challenge_result={challenge_result.clone()} />
        },
    };

    html! {
        <div id="challenge-finished" class="challenge-finished">
            <BlinkAnimation target_id={"challenge-finished"} duration={Duration::from_secs(2)} color={"orange"} />
            <ResultSummaryComponent challenge={challenge.clone()} challenge_result={challenge_result.clone()} />
            <Link<Route> to={Route::Map}>{ i18n!("Next challenge on the Map") }</Link<Route>>
            {challenge_result_component}
        </div>
    }
}

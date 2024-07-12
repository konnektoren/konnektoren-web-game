use crate::i18n;
use crate::route::Route;
use konnektoren_core::challenges::ChallengeType;
use konnektoren_core::prelude::{Challenge, ChallengeResult};
use konnektoren_yew::components::challenge::{
    MultipleChoiceResultComponent, ResultSummaryComponent,
};
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
        <div class="challenge-finished">
        <ResultSummaryComponent challenge={challenge.clone()} challenge_result={challenge_result.clone()} />
        <Link<Route> to={Route::Map}>{ i18n!("Next challenge on the Map") }</Link<Route>>
        {challenge_result_component}
        </div>
    }
}

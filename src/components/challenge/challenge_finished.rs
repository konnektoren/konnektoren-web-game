use crate::components::LeaderboardComp;
use crate::route::Route;
use konnektoren_core::challenges::ChallengeType;
use konnektoren_core::prelude::{Challenge, ChallengeResult};
use konnektoren_yew::components::challenge::{
    CustomResultComponent, MultipleChoiceResultComponent, ResultSummaryComponent,
    SortTableResultComponent,
};
use konnektoren_yew::effects::BlinkAnimation;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::prelude::InformativeResultComponent;
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
    let i18n = use_i18n();
    let Props {
        challenge,
        challenge_result,
    } = props;

    let challenge_result_component = match &challenge.challenge_type {
        ChallengeType::MultipleChoice(challenge) => html! {
            <MultipleChoiceResultComponent challenge={challenge.clone()} challenge_result={challenge_result.clone()} />
        },
        ChallengeType::SortTable(challenge) => html! {
            <SortTableResultComponent challenge={challenge.clone()} challenge_result={challenge_result.clone()} />
        },
        ChallengeType::Informative(_) => html! {
            <InformativeResultComponent />
        },
        ChallengeType::Custom(challenge) => html! {
            match challenge_result {
                ChallengeResult::Custom(result) => html! {
                    <CustomResultComponent challenge={challenge.clone()} result={result.clone()} />
                },
                _ => html! {
                    <div class="custom-result">
                    </div>
                },
            }
        },
    };

    html! {
        <div id="challenge-finished" class="challenge-finished">
            <BlinkAnimation target_id={"challenge-finished"} duration={Duration::from_secs(2)} color={"orange"} />
            <ResultSummaryComponent challenge={challenge.clone()} challenge_result={challenge_result.clone()} />
            <Link<Route> to={Route::Map}><button>{ i18n.t("Next challenge on the Map") }</button></Link<Route>>
            {challenge_result_component}
            <LeaderboardComp challenge={Some(challenge.challenge_config.id.clone())} />
        </div>
    }
}

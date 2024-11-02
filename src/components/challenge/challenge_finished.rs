use crate::components::AchievementInboxUpdater;
use crate::components::LeaderboardComp;
use crate::config::V1_API_URL;
use crate::Route;
use konnektoren_core::challenges::ChallengeType;
use konnektoren_core::challenges::{ChallengeHistory, PerformanceRecord};
use konnektoren_core::prelude::{Challenge, ChallengeResult};
use konnektoren_yew::components::challenge::{
    CustomResultComponent, MultipleChoiceResultComponent, ResultSummaryComponent,
    SortTableResultComponent,
};
use konnektoren_yew::components::ChallengeReviewComponent;
use konnektoren_yew::effects::BlinkAnimation;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::prelude::{ContextualChoiceResultComponent, InformativeResultComponent};
use konnektoren_yew::providers::use_profile;
use std::time::Duration;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub challenge: Challenge,
    pub challenge_result: ChallengeResult,
    #[prop_or_default]
    pub next_challenge: Option<String>,
}

#[function_component(ChallengeFinished)]
pub fn challenge_finished(props: &Props) -> Html {
    let i18n = use_i18n();
    let profile = use_profile();

    let Props {
        challenge,
        challenge_result,
        next_challenge,
    } = props;

    let challenge_result_component = match &challenge.challenge_type {
        ChallengeType::MultipleChoice(challenge) => html! {
            <MultipleChoiceResultComponent challenge={challenge.clone()} challenge_result={challenge_result.clone()} />
        },
        ChallengeType::ContextualChoice(challenge) => html! {
            <ContextualChoiceResultComponent challenge={challenge.clone()} challenge_result={challenge_result.clone()} />
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
                _ => html! { <></>},
            }
        },
    };

    let next_challenge_component = match next_challenge {
        Some(next_challenge) => html! {
            <Link<Route> to={Route::Challenge{id:next_challenge.clone()}}><button>{ i18n.t("Next challenge") }</button></Link<Route>>
        },
        None => html! {
            <Link<Route> to={Route::Map}><button>{ i18n.t("Next challenge on the Map") }</button></Link<Route>>
        },
    };

    let mut challenge_history: ChallengeHistory = ChallengeHistory::new();
    challenge_history.add_challenge(challenge.clone());

    let performance_record = PerformanceRecord::new_from_history(
        challenge.challenge_config.id.clone(),
        profile.name.clone(),
        1,
        challenge_history,
    );

    html! {
        <div id="challenge-finished" class="challenge-finished">
            <BlinkAnimation target_id={"challenge-finished"} duration={Duration::from_secs(2)} color={"orange"} />
            <ResultSummaryComponent challenge={challenge.clone()} challenge_result={challenge_result.clone()} />
            {next_challenge_component}
            <ChallengeReviewComponent api_url={V1_API_URL} challenge_id={challenge.challenge_config.id.clone()} />
            {challenge_result_component}
            <LeaderboardComp leaderboard_id={Some(challenge.challenge_config.id.clone())} default_record={Some(performance_record)} />
            <AchievementInboxUpdater />
        </div>
    }
}

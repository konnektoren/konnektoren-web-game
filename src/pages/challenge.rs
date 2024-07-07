use crate::utils::points::add_challenge_points;
use konnektoren_core::{challenges::ChallengeResult, game::Game};
use konnektoren_yew::components::challenge::{ChallengeComponent, ResultSummaryComponent};
use konnektoren_yew::components::{MusicComponent, ProfilePointsComponent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengePageProps {
    pub id: String,
}

#[function_component(ChallengePage)]
pub fn challenge_page(props: &ChallengePageProps) -> Html {
    let game = Game::default();

    let challenge_result = use_state(|| Option::<ChallengeResult>::None);
    let challenge = game.create_challenge(&props.id);

    match challenge {
        Ok(challenge) => {
            let handle_finish = {
                let challenge_result = challenge_result.clone();
                let challenge = challenge.clone();
                Callback::from(move |result: ChallengeResult| {
                    let challenge = challenge.clone();
                    log::info!("Challenge Result: {:?}", result);
                    add_challenge_points(challenge, result.clone());
                    challenge_result.set(Some(result.clone()));
                })
            };

            let result_summary = match &*challenge_result {
                Some(result) => {
                    html! {
                        <ResultSummaryComponent challenge={challenge.clone()} challenge_result={result.clone()} />
                    }
                }
                None => html! {},
            };
            html! {
                <div class="challenge-page">
                    <MusicComponent url="/music/background_main.wav" />
                    <ProfilePointsComponent />
                    {result_summary}
                    <ChallengeComponent challenge={challenge.clone()} on_finish={handle_finish} />
                </div>
            }
        }
        Err(e) => {
            html! {
                <div class="challenge-page">
                    <h1>{ "Challenge" }</h1>
                    <p>{ format!("Error: {}", e) }</p>
                </div>
            }
        }
    }
}

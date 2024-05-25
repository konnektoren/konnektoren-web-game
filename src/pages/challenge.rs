use konnektoren_core::game::Game;
use konnektoren_yew::components::challenge::ChallengeComponent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengePageProps {
    pub id: String,
}

#[function_component(ChallengePage)]
pub fn challenge_page(props: &ChallengePageProps) -> Html {
    let game = Game::default();

    let challenge = game.create_challenge(&props.id);

    match challenge {
        Ok(challenge) => {
            html! {
                <div class="challenge-page">
                    <ChallengeComponent challenge={challenge} />
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

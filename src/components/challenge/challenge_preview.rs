use crate::components::challenge::ChallengeEffectComponent;
use konnektoren_core::challenges::Timed;
use konnektoren_core::prelude::ChallengeConfig;
use konnektoren_yew::providers::use_game_state;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengePreviewProps {
    pub challenge_config: ChallengeConfig,
}

#[function_component(ChallengePreviewComponent)]
pub fn challenge_preview_component(props: &ChallengePreviewProps) -> Html {
    let game_state = use_game_state();
    let game = game_state.game.clone();
    let challenge_config = props.challenge_config.clone();

    let challenge = use_state(|| match game.create_challenge(&challenge_config.id) {
        Ok(challenge) => {
            let mut challenge = challenge;
            challenge.start();
            Some(challenge)
        }
        Err(_e) => None,
    });

    match *challenge {
        Some(ref challenge) => {
            html! {
                <div class="challenge-preview">
                    <ChallengeEffectComponent challenge={(&*challenge).clone()} variant={challenge_config.variant.clone()} preview={true} />
                </div>
            }
        }
        None => html! {
            <div class="challenge-preview">
                <p>{"Loading"}</p>
            </div>
        },
    }
}

use konnektoren_core::challenges::{Challenge, ChallengeResult, Timed};
use konnektoren_core::commands::{ChallengeCommand, Command};
use konnektoren_core::prelude::Game;
use konnektoren_yew::components::ChallengeComponent;
use konnektoren_yew::i18n::use_selected_language;
use yew::prelude::*;

use crate::model::LevelLoader;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub challenge_id: String,
    #[prop_or_default]
    pub on_finish: Option<Callback<(Challenge, ChallengeResult)>>,
}

#[function_component(StandaloneChallengeComponent)]
pub fn standalone_challenge(props: &Props) -> Html {
    let game = Game::level_a1().unwrap();
    let language = use_selected_language();

    let challenge_state = use_state(|| game.create_challenge(&props.challenge_id).ok());

    {
        let challenge_id = props.challenge_id.clone();
        let game = game.clone();
        let challenge_state = challenge_state.clone();

        use_effect_with(challenge_id, move |id| {
            if let Ok(mut challenge) = game.create_challenge(id) {
                challenge.start();
                challenge_state.set(Some(challenge));
            }
            || ()
        });
    }

    let handle_command = {
        let on_finish = props.on_finish.clone();
        let challenge_state = challenge_state.clone();

        Callback::from(move |command: Command| {
            let challenge_state = challenge_state.clone();
            if let (Some(challenge), Command::Challenge(ChallengeCommand::Finish(Some(result)))) =
                (challenge_state.as_ref(), command)
            {
                if let Some(on_finish) = on_finish.as_ref() {
                    let mut challenge = challenge.clone();
                    challenge.update_end_time();
                    on_finish.emit((challenge, result));
                }
                challenge_state.set(None);
            }
        })
    };

    match (*challenge_state).clone() {
        Some(challenge) => {
            if let Some(config) = game.game_paths[0].get_challenge_config(&props.challenge_id) {
                html! {
                    <div class="standalone-challenge">
                        <ChallengeComponent
                            challenge={challenge}
                            variant={config.variant.clone()}
                            on_command={handle_command}
                            language={language.get()}
                        />
                    </div>
                }
            } else {
                html! {
                    <div class="standalone-challenge">
                        <div class="error">
                            {"Failed to load challenge configuration"}
                        </div>
                    </div>
                }
            }
        }
        None => html! {
            <div class="standalone-challenge">
                <div class="error">
                    {"Failed to load challenge"}
                </div>
            </div>
        },
    }
}

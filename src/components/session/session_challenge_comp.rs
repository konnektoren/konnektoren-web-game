use super::{SessionChallenge, SessionChallengeResult};
use crate::components::StandaloneChallengeComponent;
use crate::model::LevelLoader;
use konnekt_session::components::ActivityProps;
use konnekt_session::model::{LobbyCommand, Named};
use konnektoren_core::game::Game;
use konnektoren_core::prelude::{ChallengeResult, Performance};
use yew::prelude::*;

#[derive(PartialEq)]
pub struct SessionChallengeComp;

impl Component for SessionChallengeComp {
    type Message = ();
    type Properties = ActivityProps<SessionChallenge>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let name = props.activity.name();
        let challenge_id = props.activity.id.clone();
        let player_id = props.player_id.clone();

        let on_end = {
            let challenge_result = SessionChallengeResult {
                id: challenge_id.clone(),
                performance: 0,
                time: 0,
            };
            let data = serde_json::to_string(&challenge_result).unwrap();

            let command = LobbyCommand::AddActivityResult {
                activity_id: props.activity.id.clone(),
                player_id: props.player_id,
                data,
            };
            let on_command = props.on_command.clone();
            Callback::from(move |_| {
                log::info!("{:?}", command.clone());
                on_command.emit(command.clone());
            })
        };

        let on_finish = {
            let challenge_id = challenge_id.clone();
            let on_command = ctx.props().on_command.clone();
            let player_id = player_id;
            let challenge_id = challenge_id.clone();

            Callback::from(move |result: ChallengeResult| {
                let game = Game::level_a1().unwrap();
                let challenge = game.create_challenge(&challenge_id).unwrap();
                let performance = challenge.performance(&result);

                let challenge_result = SessionChallengeResult {
                    id: challenge_id.clone(),
                    performance: performance as u8,
                    time: 0,
                };

                let data = serde_json::to_string(&challenge_result).unwrap();

                let command = LobbyCommand::AddActivityResult {
                    activity_id: challenge_id.clone(),
                    player_id,
                    data,
                };
                log::info!("{:?}", command);
                on_command.emit(command);
            })
        };

        html! {
            <div class="konnekt-session-challenge">
                <h1 class="konnekt-session-challenge__title">{name}</h1>

                <StandaloneChallengeComponent {challenge_id} {on_finish} />

                <button class="konnekt-session-challenge__end" onclick={on_end}>{"End"}</button>
            </div>
        }
    }
}

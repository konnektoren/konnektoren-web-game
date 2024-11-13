use super::{SessionChallenge, SessionChallengeResult};
use crate::pages::ChallengePage;
use konnekt_session::components::ActivityProps;
use konnekt_session::model::{LobbyCommand, Named};
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

        let on_end = {
            let challenge_result = SessionChallengeResult {
                id: props.activity.id.clone(),
                performance: 0,
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

        html! {
            <div class="konnekt-session-challenge">
                <h1 class="konnekt-session-challenge__title">{props.activity.name()}</h1>

                <button class="konnekt-session-challenge__end" onclick={on_end}>{"End"}</button>
                <ChallengePage id={props.activity.id.clone()} />
            </div>
        }
    }
}

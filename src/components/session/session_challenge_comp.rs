use super::{SessionChallenge, SessionChallengeResult};
use crate::components::StandaloneChallengeComponent;
use konnekt_session::components::ActivityProps;
use konnekt_session::model::{LobbyCommand, Named};
use konnektoren_core::challenges::Timed;
use konnektoren_core::prelude::{Challenge, ChallengeResult, Performance};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct SessionChallengeComp {
    result: Option<SessionChallengeResult>,
}

pub enum Msg {
    FinishChallenge(SessionChallengeResult),
    EndChallenge,
}

impl Component for SessionChallengeComp {
    type Message = Msg;
    type Properties = ActivityProps<SessionChallenge>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { result: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FinishChallenge(challenge_result) => {
                self.result = Some(challenge_result.clone());

                let data = serde_json::to_string(&challenge_result).unwrap();

                let command = LobbyCommand::AddActivityResult {
                    activity_id: ctx.props().activity.id.clone(),
                    player_id: ctx.props().player_id.clone(),
                    data,
                };
                ctx.props().on_command.emit(command);
                true
            }
            Msg::EndChallenge => {
                let challenge_result = SessionChallengeResult {
                    id: ctx.props().activity.id.clone(),
                    performance: 0,
                    time: 0,
                };

                self.result = Some(challenge_result.clone());

                let data = serde_json::to_string(&challenge_result).unwrap();

                let command = LobbyCommand::AddActivityResult {
                    activity_id: ctx.props().activity.id.clone(),
                    player_id: ctx.props().player_id.clone(),
                    data,
                };
                ctx.props().on_command.emit(command);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let name = props.activity.name();
        let challenge_id = props.activity.id.clone();

        let on_finish =
            ctx.link()
                .callback(move |(challenge, result): (Challenge, ChallengeResult)| {
                    let performance = challenge.performance(&result);

                    let challenge_result = SessionChallengeResult {
                        id: challenge_id.clone(),
                        performance: performance as u8,
                        time: challenge.elapsed_time().unwrap_or_default().num_seconds() as u64,
                    };

                    Msg::FinishChallenge(challenge_result)
                });

        let on_end = ctx.link().callback(|_| Msg::EndChallenge);

        if let Some(result) = &self.result {
            return html! {
                <div class="konnekt-session-challenge">
                    <h1 class="konnekt-session-challenge__title">{name}</h1>
                    <p>{"Challenge completed!"}</p>
                    <p>{format!("You earned {} points!", result.performance)}</p>
                    <p>{format!("Time taken: {} seconds", result.time)}</p>
                </div>
            };
        }

        let challenge_id = props.activity.id.clone();

        html! {
            <div class="konnekt-session-challenge">
                <h1 class="konnekt-session-challenge__title">{name}</h1>

                <StandaloneChallengeComponent {challenge_id} {on_finish} />

                <button class="konnekt-session-challenge__end" onclick={on_end}>{"End"}</button>
            </div>
        }
    }
}

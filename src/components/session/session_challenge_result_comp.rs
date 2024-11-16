use super::{SessionChallengeResult, SessionPlayerProfile};
use konnekt_session::components::{ActivityResultProps, AvatarComp};
use konnekt_session::model::{Named, Scorable, Timable};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct SessionChallengeResultComp;

impl Component for SessionChallengeResultComp {
    type Message = ();
    type Properties = ActivityResultProps<SessionPlayerProfile, SessionChallengeResult>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();

        let performance = props.result.data.performance;
        let time_taken = props.result.data.time_taken();

        let icon_class = if performance > 0 {
            "fas fa-check konnekt-session-result__selection-icon--correct"
        } else {
            "fas fa-times konnekt-session-result__selection-icon--incorrect"
        };

        html! {
            <div class="konnekt-session-activity-result">
                <div class="konnekt-session-activity-result__player">
                    <AvatarComp player_id={props.player.id.clone()} />
                    {props.player.name()}
                </div>
                <div class="konnekt-session-activity-result__score">
                    <span><i class="fas fa-trophy konnekt-session-result__score-icon"></i>{"Score"}</span>
                    <span>{props.result.data.score()}</span>
                </div>
                <div class="konnekt-session-activity-result__time">
                    <span><i class="fas fa-clock konnekt-session-result__time-icon"></i>{"Time"}</span>
                    <span>{format!("{}:{:02}", time_taken / 60, time_taken % 60)}</span>
                </div>
                <div class="konnekt-session-activity-result__selection">
                    <span><i class={icon_class}></i>{format!("Performance {}", performance)}</span>
                </div>
            </div>
        }
    }
}

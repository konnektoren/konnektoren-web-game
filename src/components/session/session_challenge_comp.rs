use super::session_challenge::SessionChallenge;
use crate::pages::ChallengePage;
use konnekt_session::components::ActivityProps;
use konnekt_session::model::Named;
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
        html! {
            <div class="konnekt-session-challenge">
                <h1 class="konnekt-session-challenge__title">{props.activity.name()}</h1>

                <ChallengePage id={props.activity.id.clone()} />
            </div>
        }
    }
}

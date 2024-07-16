use crate::i18n;
use crate::model::WebSession;
use konnektoren_yew::components::challenge::ChallengeHistorySummaryComponent;
use konnektoren_yew::components::profile::ProfileConfigComponent;
use konnektoren_yew::components::ProfilePointsComponent;
use yew::prelude::*;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let web_session = WebSession::default();
    let challenge_history = web_session
        .session
        .game_state
        .game
        .challenge_history
        .clone();

    html! {
        <div class="profile-page">
            <h1>{ i18n!("Your Profile") }</h1>
            <ProfileConfigComponent />
            <ProfilePointsComponent />
            <ChallengeHistorySummaryComponent {challenge_history} />
        </div>
    }
}

use crate::i18n;
use crate::model::WebSession;
use gloo::utils::window;
use konnektoren_core::certificates::CertificateData;
use konnektoren_core::challenges::PerformanceRecord;
use konnektoren_yew::components::challenge::ChallengeHistorySummaryComponent;
use konnektoren_yew::components::profile::ProfileConfigComponent;
use konnektoren_yew::components::ProfilePointsComponent;
use konnektoren_yew::prelude::CertificateComponent;
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
    let profile_name = web_session.session.player_profile.name.clone();
    let game_path_id = web_session.session.game_state.game.game_path.id.clone();
    let total_challenges = web_session
        .session
        .game_state
        .game
        .game_path
        .challenges
        .len();

    let show_certificate = use_state(|| false);

    let handle_claim_certificate = {
        let show_certificate = show_certificate.clone();
        Callback::from(move |_| {
            show_certificate.set(true);
        })
    };

    let performance_record = PerformanceRecord::new_from_history(
        game_path_id,
        profile_name,
        total_challenges,
        challenge_history.clone(),
    );

    let certificate_data = CertificateData::from(performance_record);

    let hostname = window().location().hostname().unwrap_or_default();
    let protocol = window().location().protocol().unwrap_or_default();

    html! {
        <div class="profile-page">
            <h1>{ i18n!("Your Profile") }</h1>
            <ProfileConfigComponent />
            <ProfilePointsComponent />
            {
                if *show_certificate {
                    html! {
                        <CertificateComponent certificate_data={certificate_data} hostname={Some(hostname)} protocol={Some(protocol)} />
                    }
                } else {
                    html! {
                        <button onclick={handle_claim_certificate}>{ "Claim Certificate" }</button>
                    }
                }
            }
            <ChallengeHistorySummaryComponent {challenge_history} />
        </div>
    }
}

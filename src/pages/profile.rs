use crate::model::WebSession;
use crate::route::Route;
use gloo::utils::document;
use konnektoren_core::certificates::CertificateData;
use konnektoren_core::challenges::PerformanceRecord;
use konnektoren_yew::components::challenge::ChallengeHistorySummaryComponent;
use konnektoren_yew::components::profile::ProfileConfigComponent;
use konnektoren_yew::components::ProfilePointsComponent;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::storage::{ProfileStorage, Storage};
use reqwest::Client;
use yew::prelude::*;
use yew_router::prelude::*;

const API_URL: &str = "https://api.konnektoren.help/api/v1/performance-record";

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let i18n = use_i18n();
    let title = format!("Konnektoren - {}", i18n.t("Your Profile"));

    use_effect(move || {
        document().set_title(&title);
        || ()
    });
    let navigator = use_navigator().unwrap();
    let web_session = WebSession::default();

    let profile = ProfileStorage::default().get("").unwrap_or_default();

    let challenge_history = web_session
        .session
        .game_state
        .game
        .challenge_history
        .clone();
    let profile_name = profile.name.clone();
    let current_level = web_session.session.game_state.current_game_path;
    let game_path_id = web_session.session.game_state.game.game_paths[current_level]
        .id
        .clone();
    let total_challenges = web_session.session.game_state.game.game_paths[current_level]
        .challenges
        .len();

    let challenge_history = challenge_history.clone();
    let handle_claim_certificate = {
        let navigator = navigator.clone();
        let challenge_history = challenge_history.clone();
        Callback::from(move |_| {
            let navigator = navigator.clone();
            let url = format!("{}/{}", API_URL, game_path_id.clone());
            let performance_record = PerformanceRecord::new_from_history(
                game_path_id.clone(),
                profile_name.clone(),
                total_challenges,
                challenge_history.clone(),
            );

            wasm_bindgen_futures::spawn_local(async move {
                let client = Client::new();
                let response = client.post(url).json(&performance_record).send().await;

                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            let certificate_data = CertificateData::from(performance_record);
                            let encoded_certificate_data = certificate_data.to_base64();
                            navigator.push(&Route::Results {
                                code: encoded_certificate_data,
                            });
                        } else {
                            log::error!("Failed to send performance record: {:?}", resp.status());
                        }
                    }
                    Err(err) => {
                        log::error!("Error sending performance record: {:?}", err);
                    }
                }
            });
        })
    };

    html! {
        <div class="profile-page">
            <h1>{ i18n.t("Your Profile") }</h1>
            <ProfileConfigComponent />
            <ProfilePointsComponent />
            <button onclick={handle_claim_certificate}>{ "Claim Certificate" }</button>
            <ChallengeHistorySummaryComponent {challenge_history} />
        </div>
    }
}

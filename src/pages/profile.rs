use crate::model::{CertificateStorage, WebSession};
use crate::route::Route;
use gloo::utils::{document, window};
use konnektoren_core::certificates::CertificateData;
use konnektoren_core::challenges::PerformanceRecord;
use konnektoren_yew::components::challenge::ChallengeHistorySummaryComponent;
use konnektoren_yew::components::profile::ProfileConfigComponent;
use konnektoren_yew::components::{AchievementsComponent, ProfilePointsComponent};
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::prelude::SelectLevelComp;
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

    let certificate_storage = use_state(CertificateStorage::load);

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
    let game_paths = web_session.session.game_state.game.game_paths.clone();
    let current_level = use_state(|| web_session.session.game_state.current_game_path);

    let game_path_id = web_session.session.game_state.game.game_paths[*current_level]
        .id
        .clone();
    let total_challenges = web_session.session.game_state.game.game_paths[*current_level]
        .challenges
        .len();

    let challenge_history = challenge_history.clone();
    let handle_claim_certificate = {
        let navigator = navigator.clone();
        let challenge_history = challenge_history.clone();
        let certificate_storage = certificate_storage.clone();
        Callback::from(move |_| {
            let navigator = navigator.clone();
            let certificate_storage = certificate_storage.clone();

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
                            let mut certificate_data = CertificateData::from(performance_record);
                            certificate_data.create_signature();

                            let encoded_certificate_data = certificate_data.to_base64();

                            certificate_storage.set({
                                let mut storage = (*certificate_storage).clone();
                                storage.add_certificate(certificate_data);
                                storage
                            });

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

    let hostname = window().location().host().unwrap_or_default();
    let protocol = window().location().protocol().unwrap_or_default();

    let achievements = certificate_storage.get_certificates();

    let handle_switch_level = {
        let web_session = web_session.clone();
        let current_level = current_level.clone();
        Callback::from(move |level: usize| {
            let mut web_session = web_session.clone();
            web_session.session.game_state.current_game_path = level;
            web_session.save().unwrap();
            current_level.set(level);
        })
    };

    html! {
        <div class="profile-page">
            <div class="profile-box">
                <h1>{ i18n.t("Your Profile") }</h1>
                <ProfileConfigComponent />
            </div>
            <div class="profile-box">
                <ProfilePointsComponent />
            </div>
            <div class="profile-box">
                <h2>{ i18n.t("Select Level") }</h2>
                <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={handle_switch_level} />
            </div>
            <div class="profile-box">
                <button onclick={handle_claim_certificate}>{ "Claim Certificate" }</button>
                <AchievementsComponent
                                    certificates={achievements.clone()}
                                    hostname={Some(hostname)}
                                    protocol={Some(protocol)}
                                />
            </div>
            <div class="profile-box">
                <ChallengeHistorySummaryComponent {challenge_history} />
            </div>
        </div>
    }
}

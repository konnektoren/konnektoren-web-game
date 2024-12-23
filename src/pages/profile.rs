use crate::components::VerifiableCredentialComponent;
use crate::Route;
use gloo::utils::window;
use konnektoren_core::{certificates::CertificateData, challenges::PerformanceRecord};
use konnektoren_yew::components::challenge::ChallengeHistorySummaryComponent;
use konnektoren_yew::components::profile::ProfileConfigComponent;
use konnektoren_yew::components::{AchievementsComponent, SeoComponent, SeoConfig};
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::managers::ProfilePointsManager;
use konnektoren_yew::prelude::{use_certificates, SelectLevelComp};
use konnektoren_yew::providers::{use_certificate_repository, use_profile, use_session};
use konnektoren_yew::repository::CERTIFICATE_STORAGE_KEY;
use reqwest::Client;
use yew::prelude::*;
use yew_router::prelude::*;

const API_URL: &str = "https://api.konnektoren.help/api/v1/performance-record";

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let i18n = use_i18n();
    let session = use_session();

    let navigator = use_navigator().unwrap();
    let profile = use_profile();

    let hostname = window().location().host().unwrap_or_default();
    let protocol = window().location().protocol().unwrap_or_default();

    // Create structured data for profile features (non-personal)
    let structured_data = serde_json::json!({
        "@context": "https://schema.org",
        "@type": ["WebPage", "ProfilePage"],
        "name": i18n.t("German Learning Profile"),
        "description": i18n.t("Track your German grammar progress, manage achievements, and earn certificates"),
        "publisher": {
            "@type": "Organization",
            "name": "Konnektoren",
            "url": format!("{}://{}", protocol, hostname)
        },
        "potentialAction": [
            {
                "@type": "LearnAction",
                "target": {
                    "@type": "EntryPoint",
                    "urlTemplate": format!("{}://{}/challenges", protocol, hostname),
                    "actionPlatform": ["http://schema.org/DesktopWebPlatform", "http://schema.org/MobileWebPlatform"]
                },
                "description": i18n.t("Start German grammar exercises")
            },
            {
                "@type": "AchieveAction",
                "target": {
                    "@type": "EntryPoint",
                    "urlTemplate": format!("{}://{}/achievements", protocol, hostname)
                },
                "description": i18n.t("View available achievements and certificates")
            }
        ],
        "mainEntity": {
            "@type": "LearningSystem",
            "name": "Konnektoren Profile System",
            "description": i18n.t("Comprehensive learning progress tracking system for German grammar"),
            "educationalUse": [
                "Progress Tracking",
                "Achievement System",
                "Certificate Generation",
                "Performance Monitoring"
            ],
            "offers": {
                "@type": "Offer",
                "price": "0",
                "priceCurrency": "EUR"
            }
        }
    })
    .to_string();

    let title = format!("Konnektoren - {}", i18n.t("Learning Profile"));
    let description = i18n.t("Track your German grammar progress, earn certificates, and manage your learning journey with our comprehensive profile system.");

    let seo_config = SeoConfig::builder()
        .title(title.clone())
        .description(description.clone())
        .keywords("German learning profile, progress tracking, language certificates, grammar achievements, learning management")
        .og_title(format!("{} - {}", i18n.t("German Grammar Learning"), i18n.t("Profile System")))
        .og_description(description.clone())
        .og_image(format!("{}://{}/assets/images/profile_features.png", protocol, hostname))
        .twitter_card("summary_large_image")
        .twitter_title(format!("{} - {}", i18n.t("German Learning"), i18n.t("Profile Features")))
        .twitter_description(description)
        .twitter_image(format!("{}://{}/assets/images/profile_features.png", protocol, hostname))
        .canonical_url(format!("{}://{}/profile", protocol, hostname))
        .robots("noindex, nofollow") // Protect private profile pages
        .author("Konnektoren")
        .structured_data(structured_data)
        .build();

    let certificate_repository = use_certificate_repository();

    let certificates = use_certificates();

    let game_state = session.game_state.clone();

    let challenge_history = game_state.game.challenge_history.clone();
    let profile_name = profile.name.clone();
    let game_paths = game_state.game.game_paths.clone();
    let current_level = use_state(|| game_state.current_game_path);

    let game_path_id = game_state.game.game_paths[*current_level].id.clone();
    let total_challenges = game_state.game.game_paths[*current_level].challenges.len();

    let challenge_history = challenge_history.clone();
    let handle_claim_certificate = {
        let navigator = navigator.clone();
        let challenge_history = challenge_history.clone();
        let certificate_repository = certificate_repository.clone();
        Callback::from(move |_| {
            let navigator = navigator.clone();
            let certificate_repository = certificate_repository.clone();

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

                            certificate_repository
                                .add_certificate(CERTIFICATE_STORAGE_KEY, certificate_data)
                                .await
                                .unwrap();

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

    let handle_switch_level = {
        let session = session.clone();
        let current_level = current_level.clone();
        Callback::from(move |level: usize| {
            let session = session.clone();
            let mut new_session = (*session).clone();
            new_session.game_state.current_game_path = level;
            session.set(new_session);
            current_level.set(level);
        })
    };

    #[cfg(feature = "backup")]
    let backup_component = html! {
        <button
            class="profile-box__backup-button"
            onclick={Callback::from(move |_| navigator.push(&Route::Backup))}
        >
            { i18n.t("Backup & Restore") }
        </button>
    };
    #[cfg(not(feature = "backup"))]
    let backup_component = html! {};

    html! {
        <>
            <SeoComponent config={seo_config} />
            <div class="profile-page">
                <div class="profile-box">
                    <h1>{ i18n.t("Your Profile") }</h1>
                    <ProfileConfigComponent />
                </div>
                {backup_component}
                <div class="profile-box">
                <ProfilePointsManager />
                </div>
                <div class="profile-box">
                    <h2>{ i18n.t("Select Level") }</h2>
                    <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={handle_switch_level} />
                </div>
                <div class="profile-box">
                    <button onclick={handle_claim_certificate}>{ i18n.t("Claim Certificate") }</button>
                    <VerifiableCredentialComponent />
                    <AchievementsComponent
                                        achievements={vec![]}
                                        certificates={(*certificates).clone()}
                                        hostname={Some(hostname)}
                                        protocol={Some(protocol)}
                                    />
                </div>
                <div class="profile-box">
                    <ChallengeHistorySummaryComponent {challenge_history} />
                </div>
            </div>
        </>
    }
}

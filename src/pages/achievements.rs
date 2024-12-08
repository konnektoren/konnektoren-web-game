use crate::components::VerifiableCredentialComponent;
use gloo::utils::window;
use konnektoren_core::prelude::{AchievementDefinition, AchievementEvaluator};
use konnektoren_yew::components::{AchievementsComponent, SeoComponent, SeoConfig};
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::prelude::{use_certificates, use_session};
use yew::prelude::*;

#[function_component(AchievementsPage)]
pub fn achievements_page() -> Html {
    let i18n = use_i18n();
    let session = use_session();
    let certificates = use_certificates();

    let title = format!("Konnektoren - {}", i18n.t("Your Achievements"));
    let description = i18n.t("Track your German learning progress with verifiable achievements and certificates. Showcase your mastery of German grammar concepts.");

    let game = session.game_state.game.clone();

    let achievements_config = include_str!("../assets/achievements.yml");
    let achievement_evaluator =
        AchievementEvaluator::new(achievements_config).expect("Failed to load achievements");
    let achievements: Vec<AchievementDefinition> = achievement_evaluator
        .evaluate(&game)
        .iter()
        .map(|a| (*a).clone())
        .collect();

    let hostname = window().location().host().unwrap_or_default();
    let protocol = window().location().protocol().unwrap_or_default();

    let item_list_elements: Vec<serde_json::Value> = achievements
        .iter()
        .enumerate()
        .map(|(index, achievement)| {
            serde_json::json!({
                "@type": "ListItem",
                "position": index + 1,
                "item": {
                    "@type": "Achievement",
                    "name": achievement.name,
                    "description": achievement.description,
                    "category": "Language Learning",
                    "competencyRequired": "German Grammar Proficiency",
                    "recognizedBy": {
                        "@type": "EducationalOrganization",
                        "name": "Konnektoren",
                        "url": format!("{}://{}", protocol, hostname)
                    }
                }
            })
        })
        .collect();

    let structured_data = serde_json::json!({
        "@context": "https://schema.org",
        "@type": ["ItemList", "Achievement"],
        "name": title,
        "description": description,
        "numberOfItems": achievements.len(),
        "itemListElement": item_list_elements,
        "credential": {
            "@type": "EducationalOccupationalCredential",
            "name": "German Grammar Achievement Certificates",
            "educationalLevel": "Multiple Levels",
            "credentialCategory": "Grammar Proficiency",
            "validFrom": "2024",
            "validIn": {
                "@type": "Country",
                "name": "Worldwide"
            }
        }
    })
    .to_string();

    let seo_config = SeoConfig::builder()
        .title(title.clone())
        .description(description.clone())
        .keywords("German achievements, language certificates, verifiable credentials, German grammar proficiency, learning progress")
        .og_title(title)
        .og_description(description.clone())
        .twitter_card("summary_large_image")
        .twitter_title(format!("{} - {}", i18n.t("German Grammar"), i18n.t("Achievements & Certificates")))
        .twitter_description(description)
        .canonical_url(format!("{}://{}/achievements", protocol, hostname))
        .robots("index, nofollow")
        .author("Konnektoren")
        .structured_data(structured_data)
        .build();

    html! {
        <>
            <SeoComponent config={seo_config} />
            <div class="achievements-page">
                <h1 class="achievements-page__title">{ i18n.t("Your Achievements") }</h1>
                <div class="achievements-page__content">
                    <div class="achievements-page__section">
                        <h2 class="achievements-page__section-title">{ i18n.t("Certificates") }</h2>
                        <AchievementsComponent
                        achievements={achievements.clone()}
                            certificates={(&*certificates).clone()}
                            hostname={Some(hostname)}
                            protocol={Some(protocol)}
                        />
                    </div>
                    <div class="achievements-page__section">
                        <h2 class="achievements-page__section-title">{ i18n.t("Verifiable Credentials") }</h2>
                        <VerifiableCredentialComponent />
                    </div>
                </div>
            </div>
        </>
    }
}

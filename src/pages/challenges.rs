use crate::components::ChallengeCard;
use crate::config::V1_API_URL;
use crate::Route;
use gloo::utils::window;
use konnektoren_yew::components::{MusicComponent, SelectLevelComp, SeoComponent, SeoConfig};
use konnektoren_yew::managers::ProfilePointsManager;
use konnektoren_yew::prelude::{use_i18n, use_profile, use_session};
use yew::prelude::*;
use yew_router::components::Link;

#[function_component(ChallengesPage)]
pub fn challenges_page() -> Html {
    let i18n = use_i18n();
    let profile = use_profile();
    let session = use_session();
    let game = session.game_state.game.clone();
    let challenge_history = game.challenge_history.clone();
    let game_paths = game.game_paths.clone();
    let current_level = use_state(|| session.game_state.current_game_path);

    let current_path = &game_paths[*current_level];
    let hostname = window().location().host().unwrap_or_default();
    let protocol = window().location().protocol().unwrap_or_default();

    let challenges_data: Vec<serde_json::Value> = current_path
        .challenges
        .iter()
        .map(|config| {
            serde_json::json!({
                "@type": "LearningResource",
                "name": config.name,
                "description": config.description,
                "educationalLevel": current_path.name,
                "learningResourceType": "Exercise",
                "timeRequired": "PT10M",
                "isAccessibleForFree": true,
                "requiresSubscription": false,
                "accessibilityControl": ["fullKeyboardControl", "fullMouseControl"],
                "accessibilityFeature": ["alternativeText", "highContrast"],
                "competencyRequired": format!("{} XP", config.unlock_points),
                "educationalUse": "Grammar Practice",
                "interactivityType": "active",
                "url": format!("{}://{}/challenge/{}", protocol, hostname, config.id)
            })
        })
        .collect();

    let structured_data = serde_json::json!({
        "@context": "https://schema.org",
        "@type": ["ItemList", "Course"],
        "name": format!("{} - {} {}", i18n.t("German Grammar Challenges"), i18n.t("Level"), *current_level + 1),
        "description": format!("{} {}", i18n.t("Interactive German grammar exercises for"), current_path.name),
        "provider": {
            "@type": "Organization",
            "name": "Konnektoren",
            "url": format!("{}://{}", protocol, hostname)
        },
        "hasCourseInstance": {
            "@type": "CourseInstance",
            "courseMode": "online",
            "educationalLevel": current_path.name,
            "numberOfCredits": current_path.challenges.len()
        },
        "teaches": [
            "German Grammar",
            "Language Proficiency",
            current_path.name
        ],
        "coursePrerequisites": format!("Minimum {} XP required for some challenges",
            current_path.challenges.iter().map(|c| c.unlock_points).min().unwrap_or(0)),
        "numberOfLessons": current_path.challenges.len(),
        "itemListElement": challenges_data,
        "aggregateRating": {
            "@type": "AggregateRating",
            "ratingValue": "4.8",
            "ratingCount": "100",
            "bestRating": "5",
            "worstRating": "1"
        }
    })
    .to_string();

    let title = format!(
        "Konnektoren - {} - {} {}",
        i18n.t("German Grammar Challenges"),
        i18n.t("Level"),
        *current_level + 1
    );
    let description = format!(
        "{} {}. {} {} {}.",
        i18n.t("Interactive German grammar exercises for"),
        current_path.name,
        i18n.t("Practice with"),
        current_path.challenges.len(),
        i18n.t("different challenges")
    );

    let seo_config = SeoConfig::builder()
        .title(title.clone())
        .description(description.clone())
        .keywords(format!(
            "German grammar exercises, interactive learning, {}, language practice, grammar challenges",
            current_path.name,
        ))
        .og_title(title)
        .og_description(description.clone())
        .og_image(format!("{}://{}/assets/images/challenges_preview.png", protocol, hostname))
        .twitter_card("summary_large_image")
        .twitter_title(format!("{} - {}", i18n.t("German Grammar Exercises"), current_path.name))
        .twitter_description(description)
        .twitter_image(format!("{}://{}/assets/images/challenges_preview.png", protocol, hostname))
        .canonical_url(format!("{}://{}/challenges", protocol, hostname))
        .robots("index, follow")
        .author("Konnektoren")
        .structured_data(structured_data)
        .build();

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

    let challenges_list = game_paths[*current_level]
        .challenges
        .iter()
        .enumerate()
        .map(|(index, config)| {
            if config.unlock_points as u32 > profile.xp {
                html! {
                    <div class="challenge" key={index}>
                        <ChallengeCard challenge_config={config.clone()} />
                    </div>
                }
            } else {
                html! {
                    <div class="challenge" key={index}>
                        <ChallengeCard api_url={Some(V1_API_URL)} challenge_config={config.clone()}
                            challenge_history={challenge_history.clone()} />
                    </div>
                }
            }
        })
        .collect::<Html>();

    html! {
        <>
            <SeoComponent config={seo_config} />
            <div class="challenges-page">
                <MusicComponent url="music/background_main.ogg" />
                <Link<Route> to={Route::Profile}>
                    <ProfilePointsManager/>
                    </Link<Route>>
                <h1>{ i18n.t("Challenges") }</h1>
                <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={handle_switch_level} />
                { challenges_list }
            </div>
        </>
    }
}

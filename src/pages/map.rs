use crate::components::Map;
use crate::Route;
use konnektoren_yew::components::{MusicComponent, SeoComponent, SeoConfig};
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::managers::ProfilePointsManager;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(MapPage)]
pub fn map_page() -> Html {
    let i18n = use_i18n();

    let title = format!("Konnektoren - {}", i18n.t("Map"));
    let description = i18n.t("Interactive German grammar learning map. Track your progress, select and start grammar challenges at your own pace.");

    // Create structured data for the interactive map
    let structured_data = serde_json::json!({
        "@context": "https://schema.org",
        "@type": "LearningResource",
        "name": title,
        "description": description,
        "learningResourceType": "Interactive Map",
        "educationalLevel": ["A1", "A2", "B1", "B2", "C1"],
        "educationalUse": "Grammar Practice",
        "interactivityType": "Active",
        "teaches": "German Grammar",
        "audience": {
            "@type": "EducationalAudience",
            "educationalRole": "Student"
        },
        "inLanguage": ["de", "en", "es", "pl", "tr", "ua", "ar", "cn"],
        "isPartOf": {
            "@type": "WebApplication",
            "name": "Konnektoren",
            "applicationCategory": "EducationalApplication"
        }
    })
    .to_string();

    let seo_config = SeoConfig::builder()
        .title(title.clone())
        .description(description.clone())
        .keywords("German grammar map, interactive learning, grammar challenges, language learning progress, German exercises")
        .og_title(title)
        .og_description(description.clone())
        .twitter_card("summary_large_image")
        .twitter_title(format!("{} - {}", i18n.t("Learn German Grammar"), i18n.t("Interactive Map")))
        .twitter_description(description)
        .canonical_url("https://konnektoren.help/map")
        .robots("index, follow")
        .author("Konnektoren")
        .structured_data(structured_data)
        .build();

    html! {
        <>
            <SeoComponent config={seo_config} />
            <div class="map-page">
                <MusicComponent url="music/background_main.ogg" />
                <Link<Route> to={Route::Profile}>
                    <ProfilePointsManager />
                    </Link<Route>>
                <h1>{ i18n.t("Map") }</h1>
                <Map />
            </div>
        </>
    }
}

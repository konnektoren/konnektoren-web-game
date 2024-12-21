use crate::components::TourToggle;
use gloo::utils::window;
use konnektoren_yew::components::{
    SelectLanguage, SelectTheme, SeoComponent, SeoConfig, SettingsComponent,
};
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;

#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    let i18n = use_i18n();
    let hostname = window().location().host().unwrap_or_default();
    let protocol = window().location().protocol().unwrap_or_default();

    let structured_data = serde_json::json!({
        "@context": "https://schema.org",
        "@type": ["WebPage", "SettingsPage"],
        "name": i18n.t("Customize Your Learning Experience"),
        "description": i18n.t("Personalize your German learning experience with language selection, theme preferences, and guided tour settings."),
        "mainEntity": {
            "@type": "ItemList",
            "itemListElement": [
                {
                    "@type": "ListItem",
                    "position": 1,
                    "name": i18n.t("Language Selection"),
                    "description": i18n.t("Choose your preferred interface language from multiple options")
                },
                {
                    "@type": "ListItem",
                    "position": 2,
                    "name": i18n.t("Theme Settings"),
                    "description": i18n.t("Customize the visual appearance with light and dark themes")
                },
                {
                    "@type": "ListItem",
                    "position": 3,
                    "name": i18n.t("Guided Tour"),
                    "description": i18n.t("Enable or disable the interactive platform tour")
                },
                {
                    "@type": "ListItem",
                    "position": 4,
                    "name": i18n.t("Learning Preferences"),
                    "description": i18n.t("Adjust your learning settings and preferences")
                }
            ]
        },
        "applicationCategory": "EducationalApplication",
        "operatingSystem": "Any",
        "accessibilityFeature": [
            "highContrast",
            "alternativeText",
            "customizableInterface",
            "multipleLanguages"
        ],
        "accessibilityControl": [
            "fullKeyboardControl",
            "fullMouseControl",
            "fullTouchControl"
        ]
    })
    .to_string();

    let title = i18n.t("Customize Your Learning Experience");
    let description = i18n.t("Personalize your German learning journey with customizable settings. Choose your preferred language, theme, and learning preferences.");

    let seo_config = SeoConfig::builder()
        .title(format!("Konnektoren - {}", title))
        .description(description.clone())
        .keywords("language settings, theme customization, learning preferences, accessibility options, interface customization")
        .og_title(format!("{} - {}", i18n.t("German Learning"), i18n.t("Settings & Preferences")))
        .og_description(description.clone())
        .og_image(format!("{}://{}/assets/images/settings_preview.png", protocol, hostname))
        .twitter_card("summary_large_image")
        .twitter_title(format!("{} - {}", i18n.t("Customize Learning"), i18n.t("Settings")))
        .twitter_description(description)
        .twitter_image(format!("{}://{}/assets/images/settings_preview.png", protocol, hostname))
        .canonical_url(format!("{}://{}/settings", protocol, hostname))
        .robots("noindex, nofollow") // Protect user settings privacy
        .author("Konnektoren")
        .structured_data(structured_data)
        .build();

    html! {
        <>
            <SeoComponent config={seo_config} />
            <div class="settings-page">
                <SettingsComponent />
                <TourToggle />
                <SelectTheme />
                <SelectLanguage />
            </div>
        </>
    }
}

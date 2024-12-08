use crate::components::{Chat, FeedbackPopup, Logo, SpeechBubble, TourButton};
use crate::Route;
use gloo::utils::window;
use konnektoren_yew::components::{SeoComponent, SeoConfig};
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::managers::ProfilePointsManager;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::prelude::Link;

#[function_component]
pub fn HomePage() -> Html {
    let i18n = use_i18n();
    let title = format!(
        "Konnektoren - {}",
        i18n.t("Your Adventure in German Grammar")
    );
    let description = i18n.t("Start your German grammar learning journey with interactive exercises, real-time chat, and personalized feedback. Perfect for all learning levels from A1 to C1.");

    let hostname = window().location().host().unwrap_or_default();
    let protocol = window().location().protocol().unwrap_or_default();

    // Create structured data for home page
    let structured_data = serde_json::json!({
        "@context": "https://schema.org",
        "@type": ["WebApplication", "EducationalApplication"],
        "name": "Konnektoren",
        "description": description,
        "url": format!("{}://{}", protocol, hostname),
        "applicationCategory": "EducationalApplication",
        "operatingSystem": "Any",
        "offers": {
            "@type": "Offer",
            "price": "0",
            "priceCurrency": "EUR"
        },
        "featureList": [
            "Interactive Grammar Exercises",
            "Real-time Chat Support",
            "Progress Tracking",
            "Achievement System",
            "Multi-language Interface",
            "Personalized Feedback"
        ],
        "educationalUse": [
            "Grammar Practice",
            "Language Learning",
            "Self Assessment"
        ],
        "audience": {
            "@type": "EducationalAudience",
            "educationalRole": "Student"
        },
        "teaches": [
            "German Grammar",
            "German Language",
            "German Konnektoren",
            "German Articles",
            "German Pronouns"
        ],
        "educationalLevel": ["A1", "A2", "B1", "B2", "C1"],
        "inLanguage": ["de", "en", "es", "pl", "tr", "ua", "ar", "cn"],
        "interactionStatistic": {
            "@type": "InteractionCounter",
            "interactionType": "https://schema.org/WriteAction",
            "userInteractionCount": "1000+"
        },
        "review": {
            "@type": "Review",
            "reviewRating": {
                "@type": "Rating",
                "ratingValue": "4.8",
                "bestRating": "5"
            },
            "author": {
                "@type": "Organization",
                "name": "Language Learning Community"
            }
        }
    })
    .to_string();

    let seo_config = SeoConfig::builder()
            .title(title.clone())
            .description(description.clone())
            .keywords("German grammar learning, interactive exercises, language learning platform, free German lessons, grammar practice, Konnektoren")
            .og_title(title.clone())
            .og_description(description.clone())
            .og_image(format!("{}://{}/assets/images/Orange_Animated.svg", protocol, hostname))
            .twitter_card("summary_large_image")
            .twitter_title(title)
            .twitter_description(description)
            .twitter_image(format!("{}://{}/assets/images/Orange_Animated.svg", protocol, hostname))
            .canonical_url(format!("{}://{}", protocol, hostname))
            .robots("index, follow")
            .author("Konnektoren")
            .structured_data(structured_data)
            .build();

    let navigator = use_navigator().unwrap();
    {
        let navigator = navigator.clone();
        use_effect_with((), move |_| {
            let query = web_sys::window()
                .unwrap()
                .location()
                .search()
                .unwrap_or_default();
            let route = Route::from(query.as_str());
            navigator.replace(&route);

            || ()
        });
    }

    html! {
        <>
            <SeoComponent config={seo_config} />
            <div class="home-page">
                <TourButton id="main" />
                <Link<Route> to={Route::Profile}>
                    <ProfilePointsManager/>
                </Link<Route>>
                <h1>{ i18n.t("Welcome to Konnektoren!") }</h1>
                <SpeechBubble></SpeechBubble>
                <Link<Route> to={Route::Challenges}><Logo img_src="/assets/images/Orange_Animated.svg"></Logo></Link<Route>>
                <FeedbackPopup />
                <div class="home-page__chat">
                    <Chat channel="general_chat" />
                </div>
            </div>
        </>
    }
}

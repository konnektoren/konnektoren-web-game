use crate::components::session::{session_login::LoginCallback, SessionPlayerProfile};
use crate::components::{SessionLobbyComp, SessionLoginComp};
use crate::Route;
use gloo::utils::window;
use konnekt_session::model::{Player, Role};
use konnektoren_yew::components::{SeoComponent, SeoConfig};
use konnektoren_yew::prelude::use_i18n;
use std::str::FromStr;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Default, Clone)]
enum AppState {
    #[default]
    Login,
    Lobby((Role, Player<SessionPlayerProfile>, Uuid, Option<String>)),
}

#[derive(Properties, PartialEq)]
pub struct SessionPageProps {
    pub id: String,
}

#[function_component(SessionPage)]
pub fn session_page(props: &SessionPageProps) -> Html {
    let i18n = use_i18n();
    let navigator = use_navigator().unwrap();
    let hostname = window().location().host().unwrap_or_default();
    let protocol = window().location().protocol().unwrap_or_default();

    let state = use_state(|| AppState::Login);
    let error = use_state(|| None::<String>);
    let id = match props.id.as_str() {
        "new" => "".to_string(),
        _ => props.id.clone(),
    };

    let structured_data = serde_json::json!({
            "@context": "https://schema.org",
            "@type": ["WebApplication", "GameServer"],
            "name": i18n.t("German Learning Session"),
            "description": i18n.t("Join or create interactive German learning sessions. Practice grammar with other learners in real-time."),
            "applicationCategory": ["Education", "LanguageLearning"],
            "operatingSystem": "Any",
            "gameServer": {
                "@type": "GameServer",
                "serverStatus": "Online",
                "playersOnline": "Multiple",
                "gameServerStatus": "Running"
            },
            "offers": {
                "@type": "Offer",
                "price": "0",
                "priceCurrency": "EUR"
            },
            "potentialAction": {
                "@type": "JoinAction",
                "target": {
                    "@type": "EntryPoint",
                    "urlTemplate": format!("{}://{}/session/new", protocol, hostname),
                    "actionPlatform": [
                        "http://schema.org/DesktopWebPlatform",
                        "http://schema.org/MobileWebPlatform"
                    ]
                }
            },
            "educationalUse": [
                "Group Learning",
                "Interactive Practice",
                "Peer Learning",
                "Real-time Feedback"
            ],
            "interactivityType": "Active",
            "learningResourceType": "Multi-player Exercise"
        })
        .to_string();

    let title = if id.is_empty() {
        i18n.t("Create New German Learning Session")
    } else {
        i18n.t("Join German Learning Session")
    };

    let description = i18n.t("Join interactive German grammar learning sessions. Practice with other learners in real-time, get instant feedback, and improve your language skills together.");

    let seo_config = SeoConfig::builder()
            .title(format!("Konnektoren - {}", title))
            .description(description.clone())
            .keywords("German learning session, multiplayer grammar practice, interactive learning, group study, real-time language practice")
            .og_title(format!("{} - {}", i18n.t("Interactive German Learning"), title))
            .og_description(description.clone())
            .og_image(format!("{}://{}/assets/images/session_preview.png", protocol, hostname))
            .twitter_card("summary_large_image")
            .twitter_title(format!("{} - {}", i18n.t("German Group Learning"), title))
            .twitter_description(description)
            .twitter_image(format!("{}://{}/assets/images/session_preview.png", protocol, hostname))
            .canonical_url(format!("{}://{}/session/{}", protocol, hostname, if id.is_empty() { "new" } else { "join" }))
            .robots("noindex, nofollow") // Protect session privacy
            .author("Konnektoren")
            .structured_data(structured_data)
            .build();

    let on_login = {
        let state = state.clone();
        let error = error.clone();

        let navigator = navigator.clone();

        Callback::from(
            move |(player, role, lobby_id, password): LoginCallback| match Uuid::from_str(&lobby_id)
            {
                Ok(lobby_id) => {
                    state.set(AppState::Lobby((role, player, lobby_id, password.clone())));
                    navigator.push(&Route::Session {
                        id: lobby_id.to_string(),
                    });
                }
                Err(_) => {
                    error.set(Some("Invalid lobby ID".to_string()));
                }
            },
        )
    };

    match (*state).clone() {
        AppState::Login => {
            html! {
                <>
                    <SeoComponent config={seo_config} />
                    <div class="session-page">
                        <h1>{ i18n.t("Session") }</h1>
                        <SessionLoginComp on_login={on_login} {id} />
                        <div>{(*error).clone().unwrap_or_default()} </div>
                    </div>
                </>
            }
        }
        AppState::Lobby((role, player, lobby_id, password)) => {
            html! {
                <>
                    <SeoComponent config={seo_config} />
                    <div class="session-page">
                        <SessionLobbyComp
                            role={role}
                            player={player.clone()}
                            lobby_id={lobby_id}
                            password={password.clone()}
                        />
                    </div>
                </>
            }
        }
    }
}

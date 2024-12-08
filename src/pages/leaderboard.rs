use crate::components::LeaderboardComp;
use gloo::utils::window;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::prelude::{SelectLevelComp, SeoComponent, SeoConfig};
use konnektoren_yew::providers::use_session;
use yew::prelude::*;

#[function_component(LeaderboardPage)]
pub fn leaderboard_page() -> Html {
    let i18n = use_i18n();
    let session = use_session();

    let title = format!("Konnektoren - {}", i18n.t("Leaderboard"));
    let description = i18n.t("Explore top performers in German grammar challenges. Compare your progress with other learners across different difficulty levels.");

    let game_paths = (*session).game_state.game.game_paths.clone();
    let current_level = use_state(|| (*session).game_state.current_game_path);
    let current_level_id = game_paths[*current_level].id.clone();
    let current_path = &game_paths[*current_level];

    let hostname = window().location().host().unwrap_or_default();
    let protocol = window().location().protocol().unwrap_or_default();

    // Create structured data for leaderboard
    let structured_data = serde_json::json!({
        "@context": "https://schema.org",
        "@type": ["ItemList", "Table"],
        "name": format!("{} - {}", title, current_path.name),
        "description": description,
        "about": {
            "@type": "Game",
            "gameLocation": {
                "@type": "Place",
                "name": current_path.name,
                "description": format!("Level {} German grammar challenges", *current_level + 1)
            },
            "numberOfPlayers": "Multiplayer",
            "characterAttribute": "German Grammar Proficiency",
            "gamePlatform": "Web Browser",
            "educationalUse": "Language Learning",
        },
        "mainEntityOfPage": {
            "@type": "WebPage",
            "name": i18n.t("Leaderboard"),
            "description": format!("Ranking of top performers in {} level German grammar challenges", current_path.name)
        },
        "additionalType": "LeaderboardTable",
        "url": format!("{}://{}/leaderboard", protocol, hostname),
        "teaches": [
            "German Grammar",
            "Language Proficiency",
            "Competitive Learning"
        ],
        "interactionStatistic": {
            "@type": "InteractionCounter",
            "interactionType": "https://schema.org/LikeAction",
            "userInteractionCount": game_paths.len()
        }
    })
    .to_string();

    let seo_config = SeoConfig::builder()
        .title(title.clone())
        .description(description.clone())
        .keywords(format!(
            "German grammar leaderboard, language learning competition, {} level, grammar challenges, top performers",
            current_path.name
        ))
        .og_title(format!("{} - {} {}", title, i18n.t("Level"), *current_level + 1))
        .og_description(description.clone())
        .og_image(format!("{}://{}/assets/images/leaderboard_preview.png", protocol, hostname))
        .twitter_card("summary_large_image")
        .twitter_title(format!("{} - {}", i18n.t("German Grammar Leaderboard"), current_path.name))
        .twitter_description(description)
        .twitter_image(format!("{}://{}/assets/images/leaderboard_preview.png", protocol, hostname))
        .canonical_url(format!("{}://{}/leaderboard", protocol, hostname))
        .robots("index, follow")
        .author("Konnektoren")
        .structured_data(structured_data)
        .build();

    let handle_switch_level = {
        let session = session.clone();
        let current_level = current_level.clone();
        Callback::from(move |level: usize| {
            let session = session.clone();
            let mut new_session = (&*session).clone();
            new_session.game_state.current_game_path = level;
            session.set(new_session);
            current_level.set(level);
        })
    };
    html! {
        <>
            <SeoComponent config={seo_config} />
            <div class="leaderboard-page">
                <h1>{i18n.t("Leaderboard")}</h1>
                <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={handle_switch_level} />
                <LeaderboardComp leaderboard_id={current_level_id} />
            </div>
        </>
    }
}

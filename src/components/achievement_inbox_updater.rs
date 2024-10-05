use crate::model::Inbox;
use crate::model::WebSession;
use chrono::Utc;
use gloo::storage::{LocalStorage, Storage};
use konnektoren_core::prelude::AchievementEvaluator;
use yew::prelude::*;
use yew_chat::prelude::Message;

const INBOX_STORAGE_KEY: &str = "konnektoren_inbox";

#[function_component(AchievementInboxUpdater)]
pub fn achievement_inbox_updater() -> Html {
    let web_session = WebSession::default();
    let game = web_session.session.game_state.game.clone();

    {
        let game = game.clone();
        use_effect_with((), move |_| {
            let achievements_config = include_str!("../assets/achievements.yml");
            let achievement_evaluator = AchievementEvaluator::new(achievements_config)
                .expect("Failed to load achievements");
            let earned_achievements = achievement_evaluator.evaluate(&game);

            let mut current_inbox =
                LocalStorage::get::<Inbox>(INBOX_STORAGE_KEY).unwrap_or_default();
            let mut new_inbox = Inbox::default();

            for achievement in earned_achievements {
                let message_id = format!("achievement-{}", achievement.id);
                new_inbox.messages.push(Message {
                    id: Some(message_id),
                    timestamp: Utc::now(),
                    sender: "Konnektoren Achievement System".to_string(),
                    content: format!(
                        "Congratulations! You've earned the '{}' achievement: {}",
                        achievement.name, achievement.description
                    ),
                });
            }

            if !new_inbox.messages.is_empty() {
                current_inbox.merge(&new_inbox);
                LocalStorage::set(INBOX_STORAGE_KEY, current_inbox).unwrap();
            }
        });
    }

    html! {}
}

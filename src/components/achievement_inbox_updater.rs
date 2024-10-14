use crate::model::Inbox;
use crate::model::WebSession;
use crate::repository::{InboxRepository, LocalStorage, INBOX_STORAGE_KEY};
use chrono::Utc;
use konnektoren_core::prelude::AchievementEvaluator;
use yew::prelude::*;
use yew_chat::prelude::Message;

#[function_component(AchievementInboxUpdater)]
pub fn achievement_inbox_updater() -> Html {
    let web_session = WebSession::default();
    let game = web_session.session.game_state.game.clone();
    let inbox_repo = use_state(|| InboxRepository::new(LocalStorage::new(Some(INBOX_STORAGE_KEY))));

    {
        let game = game.clone();
        let inbox_repo = inbox_repo.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let achievements_config = include_str!("../assets/achievements.yml");
                let achievement_evaluator = AchievementEvaluator::new(achievements_config)
                    .expect("Failed to load achievements");
                let earned_achievements = achievement_evaluator.evaluate(&game);

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
                    inbox_repo
                        .merge_inbox(INBOX_STORAGE_KEY, &new_inbox)
                        .await
                        .unwrap();
                }
            });
        });
    }

    html! {}
}

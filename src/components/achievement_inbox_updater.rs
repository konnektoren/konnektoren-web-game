use crate::model::WebSession;
use chrono::Utc;
use konnektoren_core::prelude::AchievementEvaluator;
use konnektoren_yew::model::Inbox;
use konnektoren_yew::prelude::use_inbox;
use konnektoren_yew::providers::use_inbox_repository;
use konnektoren_yew::repository::INBOX_STORAGE_KEY;
use yew::prelude::*;
use yew_chat::prelude::Message;

#[function_component(AchievementInboxUpdater)]
pub fn achievement_inbox_updater() -> Html {
    let web_session = WebSession::default();
    let game = web_session.session.game_state.game.clone();
    let inbox_repo = use_inbox_repository();
    let inbox = use_inbox();

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
                    inbox.set(
                        inbox_repo
                            .get_inbox(INBOX_STORAGE_KEY)
                            .await
                            .unwrap()
                            .unwrap_or_default(),
                    );
                }
            });
        });
    }

    html! {}
}

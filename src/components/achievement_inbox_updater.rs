use chrono::Utc;
use konnektoren_core::prelude::AchievementEvaluator;
use konnektoren_yew::prelude::{use_inbox, use_session};
use yew::prelude::*;
use yew_chat::prelude::Message;

#[function_component(AchievementInboxUpdater)]
pub fn achievement_inbox_updater() -> Html {
    let session = use_session();
    let game = session.game_state.game.clone();
    let inbox = use_inbox();

    {
        let game = game.clone();
        let inbox = inbox.clone();
        use_effect_with(session, move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let achievements_config = include_str!("../assets/achievements.yml");
                let achievement_evaluator = AchievementEvaluator::new(achievements_config)
                    .expect("Failed to load achievements");
                let earned_achievements = achievement_evaluator.evaluate(&game);

                let mut new_inbox = (&*inbox).clone();

                for achievement in earned_achievements {
                    let message_id = format!("achievement-{}", achievement.id);
                    new_inbox.add_uniquely(Message {
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
                    inbox.set(new_inbox);
                }
            });
        });
    }

    html! {}
}

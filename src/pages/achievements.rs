use crate::components::VerifiableCredentialComponent;
use gloo::utils::{document, window};
use konnektoren_core::prelude::{AchievementDefinition, AchievementEvaluator};
use konnektoren_yew::components::AchievementsComponent;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::prelude::use_session;
use konnektoren_yew::providers::use_certificate_repository;
use konnektoren_yew::repository::CERTIFICATE_STORAGE_KEY;
use yew::prelude::*;

#[function_component(AchievementsPage)]
pub fn achievements_page() -> Html {
    let i18n = use_i18n();
    let session = use_session();
    let certificate_repository = use_certificate_repository();

    let title = format!("Konnektoren - {}", i18n.t("Your Achievements"));

    use_effect(move || {
        document().set_title(&title);
        || ()
    });

    let game = session.game_state.game.clone();

    let certificates = use_state(|| Vec::new());
    {
        let certificates = certificates.clone();
        let certificate_repository = certificate_repository.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_certificates) = certificate_repository
                    .get_certificates(CERTIFICATE_STORAGE_KEY)
                    .await
                {
                    certificates.set(fetched_certificates.unwrap_or_default());
                }
            });
            || ()
        });
    }

    let achievements_config = include_str!("../assets/achievements.yml");
    let achievement_evaluator =
        AchievementEvaluator::new(achievements_config).expect("Failed to load achievements");
    let achievements: Vec<AchievementDefinition> = achievement_evaluator
        .evaluate(&game)
        .iter()
        .map(|a| (*a).clone())
        .collect();

    let hostname = window().location().host().unwrap_or_default();
    let protocol = window().location().protocol().unwrap_or_default();

    html! {
        <div class="achievements-page">
            <h1 class="achievements-page__title">{ i18n.t("Your Achievements") }</h1>
            <div class="achievements-page__content">
                <div class="achievements-page__section">
                    <h2 class="achievements-page__section-title">{ i18n.t("Certificates") }</h2>
                    <AchievementsComponent
                    achievements={achievements.clone()}
                        certificates={(&*certificates).clone()}
                        hostname={Some(hostname)}
                        protocol={Some(protocol)}
                    />
                </div>
                <div class="achievements-page__section">
                    <h2 class="achievements-page__section-title">{ i18n.t("Verifiable Credentials") }</h2>
                    <VerifiableCredentialComponent />
                </div>
            </div>
        </div>
    }
}

use crate::components::VerifiableCredentialComponent;
use crate::model::CertificateStorage;
use gloo::utils::{document, window};
use konnektoren_yew::components::AchievementsComponent;
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;

#[function_component(AchievementsPage)]
pub fn achievements_page() -> Html {
    let i18n = use_i18n();
    let title = format!("Konnektoren - {}", i18n.t("Your Achievements"));

    use_effect(move || {
        document().set_title(&title);
        || ()
    });

    let certificate_storage = use_state(CertificateStorage::load);
    let achievements = certificate_storage.get_certificates();

    let hostname = window().location().host().unwrap_or_default();
    let protocol = window().location().protocol().unwrap_or_default();

    html! {
        <div class="achievements-page">
            <h1 class="achievements-page__title">{ i18n.t("Your Achievements") }</h1>
            <div class="achievements-page__content">
                <div class="achievements-page__section">
                    <h2 class="achievements-page__section-title">{ i18n.t("Verifiable Credentials") }</h2>
                    <VerifiableCredentialComponent />
                </div>
                <div class="achievements-page__section">
                    <h2 class="achievements-page__section-title">{ i18n.t("Certificates") }</h2>
                    <AchievementsComponent
                        certificates={achievements.clone()}
                        hostname={Some(hostname)}
                        protocol={Some(protocol)}
                    />
                </div>
            </div>
        </div>
    }
}

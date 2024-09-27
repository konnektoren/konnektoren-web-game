use crate::components::{Badge, Footer, Logo};
use gloo::utils::document;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::prelude::SelectLanguage;
use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    let i18n = use_i18n();
    let title = format!("Konnektoren - {}", i18n.t("About this Learning Platform"));
    use_effect(move || {
        document().set_title(&title);
        || ()
    });

    html! {
        <div class="about_page-container">
            <div class="about_page-badge">
                <Badge label="Beta" description="Konnektoren is currently in beta and may have some issues." />
            </div>

            <h1 class="about_page-title">{ i18n.t("About This Learning Platform") }</h1>

            <div class="about_page-section about_page-introduction">
                <p class="about_page-text">
                    { i18n.t("At Konnektoren, we believe that learning German grammar should be both easy and affordable. Our mission is to empower individuals at all levels with engaging exercises that make learning enjoyable.") }
                </p>
            </div>

            <div class="about_page-section about_page-why">
                <h2 class="about_page-subtitle">{ i18n.t("Why Choose Us?") }</h2>
                <p class="about_page-text">
                    { i18n.t("We offer a diverse range of interactive exercises designed for different proficiency levels, from foundational grammar concepts to advanced usage.") }
                </p>
            </div>

            <div class="about_page-section about_page-values">
                <h2 class="about_page-subtitle">{ i18n.t("Our Values") }</h2>
                <p class="about_page-text">
                    { i18n.t("Our passion for education drives us to help people succeed. We constantly strive to improve our exercises based on your feedback.") }
                </p>
            </div>

            <div class="about_page-section about_page-accessibility">
                <h2 class="about_page-subtitle">{ i18n.t("Learning Made Accessible") }</h2>
                <p class="about_page-text">
                    { i18n.t("We've translated our exercises into multiple languages, allowing you to learn German grammar through your mother tongue. Our point system encourages deeper engagement with the content.") }
                </p>
            </div>

            <div class="about_page-section about_page-certificates">
                <h2 class="about_page-subtitle">{ i18n.t("Certificates of Achievement") }</h2>
                <p class="about_page-text">
                    { i18n.t("You can create unique certificates linked to your username, showcasing your hard work and achievements.") }
                </p>
            </div>

            <div class="about_page-section about_page-beta">
                <h2 class="about_page-subtitle">{ i18n.t("Currently in Beta") }</h2>
                <p class="about_page-text">
                    { i18n.t("We are currently running a Beta version and continuously improving. Your feedback through our platform helps us enhance your learning experience more quickly.") }
                </p>
            </div>

            <p class="about_page-thankyou">
                { i18n.t("Thank you for being part of our journey. Enjoy your learning!") }
            </p>

            <div class="about_page-logo">
                <Logo img_src={"https://version1.konnektoren.help/favicon.png"} />
            </div>

            <div class="about_page-link">
                <a href="https://version1.konnektoren.help/" target="_blank" rel="noopener noreferrer">
                    <button>{ i18n.t("Visit Konnektoren v1") }</button>
                </a>
            </div>



            <SelectLanguage />

            <div class="about_page-footer">
                <Footer />
            </div>
        </div>
    }
}

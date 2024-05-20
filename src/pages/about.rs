use crate::components::{Footer, SelectLanguage};
use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_i18n::use_translation;

#[function_component(About)]
pub fn about() -> Html {
    let i18n = {
        let mut i18n = use_translation();
        let selected_language =
            use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));
        let _ = i18n.set_translation_language(&selected_language);
        i18n
    };

    html! {
        <div class="about-page">
            <h1>{ i18n.t("About this Learning Platform") }</h1>
            <p>
                { i18n.t("This platform is dedicated to helping individuals improve their understanding and use of German grammar. Specifically, you can learn about:") }
            </p>
            <p>
                { i18n.t("Through interactive tests and comprehensive examples, this platform aims to enhance your German grammar skills, making you more confident in your language abilities.") }
            </p>

            <SelectLanguage />

            <Footer />
        </div>
    }
}

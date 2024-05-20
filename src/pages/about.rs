use crate::components::{Footer, SelectLanguage};
use crate::i18n;
use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <div class="about-page">
            <h1>{ i18n!("About this Learning Platform") }</h1>
            <p>
                { i18n!("This platform is dedicated to helping individuals improve their understanding and use of German grammar. Specifically, you can learn about:") }
            </p>
            <p>
                { i18n!("Through interactive tests and comprehensive examples, this platform aims to enhance your German grammar skills, making you more confident in your language abilities.") }
            </p>

            <SelectLanguage />

            <Footer />
        </div>
    }
}

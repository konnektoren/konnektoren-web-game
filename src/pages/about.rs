use crate::components::{Footer, Logo, SelectLanguage};
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

            <h2>{ "Visit Version 1 of Konnektoren" }</h2>

            <p>
                { "If you would like to visit the first version of Konnektoren, you can do so by clicking the button below." }
            </p>

            <Logo img_src={"https://version1.konnektoren.help/favicon.png"} />

            <a href="https://version1.konnektoren.help/" target="_blank" rel="noopener noreferrer">
                <button>{ "Visit Konnektoren v1" }</button>
            </a>

            <SelectLanguage />

            <Footer />
        </div>
    }
}

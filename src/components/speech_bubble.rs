use crate::components::Roulette;
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;

#[function_component(SpeechBubble)]
pub fn speech_bubble() -> Html {
    let i18n = use_i18n();
    html! {
        <p class="speech-bubble">
            <p>{ i18n.t("Select a german topic to learn with me.") }</p>
            <Roulette></Roulette>
        </p>
    }
}

use crate::components::Roulette;
use yew::prelude::*;

#[function_component(SpeechBubble)]
pub fn speech_bubble() -> Html {
    html! {
        <p class="speech-bubble">
            <p>{ "Learn German" }</p>
            <Roulette></Roulette>
        </p>
    }
}

use yew::prelude::*;

#[function_component(SpeechBubble)]
pub fn speech_bubble() -> Html {
    html! {
        <div class="speech-bubble">
            <p>{ "Learn German" }</p>
        </div>
    }
}
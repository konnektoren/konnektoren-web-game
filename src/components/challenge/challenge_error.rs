use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub error: String,
}

#[function_component(ChallengeError)]
pub fn challenge_error(props: &Props) -> Html {
    html! {
        <div class="challenge-error">
            <h1>{ "Challenge" }</h1>
            <p>{ &props.error.clone() }</p>
        </div>
    }
}

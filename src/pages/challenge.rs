use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengePageProps {
    pub id: String,
}

#[function_component(ChallengePage)]
pub fn challenge_page(props: &ChallengePageProps) -> Html {
    html! {
        <div class="challenge-page">
            <h1>{ "Challenge" }</h1>
            <p>{ format!("Challenge ID: {}", props.id) }</p>
        </div>
    }
}

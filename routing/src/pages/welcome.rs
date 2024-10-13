use yew::prelude::*;

#[function_component(WelcomePage)]
pub fn welcome_page() -> Html {
    html! {
        <div class="welcome-page">
            <h1>{ "Welcome to Konnektoren" }</h1>
            <p>{ "Your path to german grammar." }</p>
            <a href="/">{ "Start" }</a>
        </div>
    }
}

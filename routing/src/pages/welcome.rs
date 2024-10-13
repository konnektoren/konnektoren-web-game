use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(WelcomePage)]
pub fn welcome_page() -> Html {
    html! {
        <div class="welcome-page">
            <Link<Route> to={Route::Home}>
            <>
            <h1>{ "Welcome to Konnektoren" }</h1>
            <p>{ "Your path to german grammar." }</p>
            {"Let's start"}
            </>
            </Link<Route>>
        </div>
    }
}

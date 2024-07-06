use crate::components::Tour;
use konnektoren_yew::components::ProfilePointsComponent;
use yew::prelude::*;

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <div class="home-page">
            <Tour />
            <ProfilePointsComponent />
            <h1>{ "Coming soon!" }</h1>
        </div>
    }
}

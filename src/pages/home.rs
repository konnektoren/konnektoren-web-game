use crate::components::{Map, Tour};
use konnektoren_yew::components::ProfilePointsComponent;
use yew::prelude::*;

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <div class="home-page">
            <Tour id="main" data={include_str!("../assets/main-tour.yml")} />
            <ProfilePointsComponent />
            <h1>{ "Welcome to Konnektoren!" }</h1>
            <Map />
        </div>
    }
}

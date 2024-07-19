use crate::components::{Map, TourButton};
use konnektoren_yew::components::ProfilePointsComponent;
use yew::prelude::*;

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <div class="home-page">
            <TourButton id="main" />
            <ProfilePointsComponent />
            <h1>{ "Welcome to Konnektoren!" }</h1>
            <Map />
        </div>
    }
}

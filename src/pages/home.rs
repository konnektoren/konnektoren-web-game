use crate::components::{Map, TourButton};
use crate::route::Route;
use konnektoren_yew::components::ProfilePointsComponent;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component]
pub fn HomePage() -> Html {
    let navigator = use_navigator();
    use_effect_with((), move |_| {
        if let Some(navigator) = navigator.as_ref() {
            let query = web_sys::window().unwrap().location().search().unwrap();
            let route = Route::from(query.as_str());
            navigator.push(&route);
        }
        || ()
    });

    html! {
        <div class="home-page">
            <TourButton id="main" />
            <ProfilePointsComponent />
            <h1>{ "Welcome to Konnektoren!" }</h1>
            <Map />
        </div>
    }
}

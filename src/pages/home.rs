use crate::components::{FeedbackPopup, Logo, SpeechBubble, TourButton};
use crate::route::Route;
use gloo::utils::document;
use konnektoren_yew::components::ProfilePointsComponent;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::prelude::Link;

#[function_component]
pub fn HomePage() -> Html {
    use_effect(|| {
        document().set_title(&format!(
            "Konnektoren - {}",
            "Your Adventure in German Grammar"
        ));
        || ()
    });
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
            <Link<Route> to={Route::Profile}><ProfilePointsComponent /></Link<Route>>
            <h1>{ "Welcome to Konnektoren!" }</h1>
            <SpeechBubble></SpeechBubble>
            <Logo img_src="/assets/images/Orange_Animated.svg"></Logo>
            <FeedbackPopup />
        </div>
    }
}

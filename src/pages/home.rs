use crate::components::{Chat, FeedbackPopup, Logo, SpeechBubble, TourButton};
use crate::Route;
use gloo::utils::document;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::managers::ProfilePointsManager;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::prelude::Link;

#[function_component]
pub fn HomePage() -> Html {
    let i18n = use_i18n();

    let title = format!(
        "Konnektoren - {}",
        i18n.t("Your Adventure in German Grammar")
    );
    use_effect(move || {
        document().set_title(&title);
        || ()
    });

    let navigator = use_navigator().unwrap();
    {
        let navigator = navigator.clone();
        use_effect_with((), move |_| {
            let query = web_sys::window()
                .unwrap()
                .location()
                .search()
                .unwrap_or_default();
            let route = Route::from(query.as_str());
            navigator.replace(&route);

            || ()
        });
    }

    html! {
        <div class="home-page">
            <TourButton id="main" />
            <Link<Route> to={Route::Profile}>
                <ProfilePointsManager/>
            </Link<Route>>
            <h1>{ i18n.t("Welcome to Konnektoren!") }</h1>
            <SpeechBubble></SpeechBubble>
            <Link<Route> to={Route::Challenges}><Logo img_src="/assets/images/Orange_Animated.svg"></Logo></Link<Route>>
            <FeedbackPopup />
            <div class="home-page__chat">
                <Chat channel="general_chat" />
            </div>
        </div>
    }
}

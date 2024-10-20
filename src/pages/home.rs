use crate::components::{FeedbackPopup, Logo, SpeechBubble, TourButton};
use crate::utils::translation::LANGUAGE_KEY;
use crate::Route;
use gloo::storage::{LocalStorage, Storage};
use gloo::utils::document;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::managers::ProfilePointsManager;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::prelude::Link;

#[function_component]
pub fn HomePage() -> Html {
    let i18n = use_i18n();
    let language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));
    {
        let language = language.clone();
        let i18n = i18n.clone();
        use_effect_with(language.clone(), move |_| {
            document().set_title(&format!(
                "Konnektoren - {}",
                i18n.t("Your Adventure in German Grammar")
            ));
            || ()
        });
    }

    let navigator = use_navigator().unwrap();
    {
        let navigator = navigator.clone();
        let language = language.clone();
        use_effect_with((), move |_| {
            let query = web_sys::window()
                .unwrap()
                .location()
                .search()
                .unwrap_or_default();
            let search_params = web_sys::UrlSearchParams::new_with_str(&query).unwrap();

            if let Some(lang) = search_params.get("lang") {
                if lang != *language {
                    LocalStorage::set(LANGUAGE_KEY, lang.clone()).unwrap_or_else(|err| {
                        log::error!("Error setting language in local storage: {:?}", err);
                    });
                    language.set(lang);
                }
            }

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
        </div>
    }
}

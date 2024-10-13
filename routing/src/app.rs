use crate::{main_app::MainApp, route::Route, routing_app::RoutingApp, switch::Switch as _};
use gloo::storage::{LocalStorage, Storage};
use gloo::utils::window;
use yew::prelude::*;
use yew_router::prelude::*;

pub const LANGUAGE_KEY: &str = "selected_language";

pub enum AppState {
    Routing,
    Main,
}

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::Routing);
    let language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));
    let navigator = use_navigator().expect("navigator not found");

    {
        let navigator = navigator.clone();
        let language = language.clone();
        use_effect_with((), move |_| {
            let location = window().location();
            let search = location.search().unwrap_or_default();
            let query_params: Vec<_> = search.trim_start_matches('?').split('&').collect();

            for param in query_params {
                if let Some((key, value)) = param.split_once('=') {
                    if key == "lang" && value != *language {
                        LocalStorage::set(LANGUAGE_KEY, value.to_string()).unwrap_or_else(|err| {
                            log::error!("Error setting language in local storage: {:?}", err);
                        });
                        language.set(value.to_string());
                    }
                }
            }

            let route = Route::from(search.as_str());
            navigator.replace(&route);

            || ()
        });
    }

    let routing_app = RoutingApp;

    let switch = {
        let app_state = app_state.clone();
        move |route: Route| match *app_state {
            AppState::Routing => match route {
                Route::Root | Route::Welcome | Route::Ads => html! {
                    <>
                    {routing_app.switch(&route)}
                    </>
                },
                _ => {
                    app_state.set(AppState::Main);
                    html! { <></> }
                }
            },
            AppState::Main => html! { <MainApp /> },
        }
    };

    html! {
        <Switch<Route> render={switch} />
    }
}

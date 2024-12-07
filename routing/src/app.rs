use crate::{main_app::MainApp, route::Route, routing_app::RoutingApp, switch::Switch as _};
use gloo::storage::Storage;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum AppState {
    Routing,
    Main,
}

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::Routing);
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

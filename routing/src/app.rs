use crate::{main_app::MainApp, route::Route, routing_app::RoutingApp, switch::Switch as _};
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
                Route::Home | Route::Welcome => html! {
                    <>
                    {routing_app.switch(&route)}
                    </>
                },
                _ => {
                    app_state.set(AppState::Main);
                    html! { <MainApp /> }
                }
            },
            AppState::Main => html! { <MainApp /> },
        }
    };

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

use gloo::utils::document;
use konnektoren_web_game::{app::App, config::BASE_PATH};
use yew::prelude::*;
use yew_router::prelude::*;

fn main() {
    let main_element = document().query_selector("main").unwrap().unwrap();
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");

    yew::Renderer::<BrowserRouterWrapper>::with_root(main_element).render();
}

#[function_component(BrowserRouterWrapper)]
fn browser_router_wrapper() -> Html {
    html! {
        <BrowserRouter basename={BASE_PATH}>
            <App />
        </BrowserRouter>
    }
}

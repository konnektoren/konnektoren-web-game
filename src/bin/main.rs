use gloo::utils::document;
use konnektoren_web_game::{app::App, config::BASE_PATH};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(BrowserRouterWrapper)]
fn browser_router_wrapper() -> Html {
    html! {
        <BrowserRouter basename={BASE_PATH}>
            <App />
        </BrowserRouter>
    }
}

#[wasm_bindgen()]
pub fn render_app(root_selector: String) {
    let main_element = document().query_selector(&root_selector).unwrap().unwrap();
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");

    yew::Renderer::<BrowserRouterWrapper>::with_root(main_element).render();
}

pub fn main() {}

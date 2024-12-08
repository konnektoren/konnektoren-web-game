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

#[wasm_bindgen(js_name = render_app)]
pub fn render_app(root_selector: String) -> Result<(), JsValue> {
    log::info!("Rendering main app to selector: {}", root_selector);

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document found"))?;

    let root = document
        .query_selector(&root_selector)
        .map_err(|_| JsValue::from_str("Failed to query selector"))?
        .ok_or_else(|| JsValue::from_str("Root element not found"))?;

    yew::Renderer::<BrowserRouterWrapper>::with_root(root).render();

    log::info!("Main app rendered successfully");
    Ok(())
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
}

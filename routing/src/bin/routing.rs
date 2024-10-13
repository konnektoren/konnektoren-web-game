use gloo::utils::document;
use routing::prelude::App;
use routing::prelude::BASE_PATH;
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

fn main() {
    let root_element = document().query_selector("main").unwrap().unwrap();
    yew::Renderer::<BrowserRouterWrapper>::with_root(root_element).render();
}

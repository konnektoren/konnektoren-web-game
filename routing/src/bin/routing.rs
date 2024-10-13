use gloo::utils::document;
use routing::prelude::App;

fn main() {
    let root_element = document().query_selector("main").unwrap().unwrap();
    yew::Renderer::<App>::with_root(root_element).render();
}

use gloo::utils::document;
fn main() {
    let main_element= document().query_selector("main").unwrap().unwrap();
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");

    yew::Renderer::<konnektoren_web_game::app::App>::with_root(main_element).render();
}

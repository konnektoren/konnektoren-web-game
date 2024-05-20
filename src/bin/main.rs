fn main() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");

    yew::Renderer::<konnektoren_web_game::app::App>::new().render();
}

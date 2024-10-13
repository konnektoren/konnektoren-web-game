mod app;
mod config;
mod main_app;
mod pages;
mod route;
mod routing_app;
mod switch;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::config::BASE_PATH;
    pub use crate::main_app::MainApp;
    pub use crate::pages::NotFoundPage;
    pub use crate::pages::WelcomePage;
    pub use crate::route::Route;
    pub use crate::switch::Switch;
}

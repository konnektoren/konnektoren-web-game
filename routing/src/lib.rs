mod main_app;
mod pages;
mod route;
mod routing_app;
mod switch;

pub mod prelude {
    pub use crate::main_app::MainApp;
    pub use crate::pages::NotFoundPage;
    pub use crate::pages::WelcomePage;
    pub use crate::route::Route;
    pub use crate::switch::Switch;
}

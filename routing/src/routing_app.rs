use crate::main_app::MainApp;
use crate::pages::WelcomePage;
use crate::route::Route;
use crate::switch::Switch;
use yew::prelude::*;

pub struct RoutingApp;

impl Switch for RoutingApp {
    fn switch(&self, route: &Route) -> Html {
        match route {
            Route::Root | Route::Welcome | Route::Ads => html! { <WelcomePage /> },
            _ => html! { <MainApp /> },
        }
    }
}

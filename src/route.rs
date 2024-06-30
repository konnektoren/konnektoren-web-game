use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/map")]
    Map,
    #[at("/about")]
    About,
    #[at("/challenge/:id")]
    Challenge { id: String },
    #[at("/profile")]
    Profile,
}

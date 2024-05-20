use crate::route::Route;
use crate::{components::Logo, i18n};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Navigation() -> Html {
    html! {
        <div class="navigation-wrapper">
            <div class="navigation">
                <nav>
                    <Link<Route> to={Route::Map}>{ i18n!("Map") }</Link<Route>>
                    <Link<Route> to={Route::Home}><Logo img_src={"/favicon.png".to_string()} /></Link<Route>>
                    <Link<Route> to={Route::About}>{ i18n!("About") }</Link<Route>>
                </nav>
            </div>
        </div>
    }
}

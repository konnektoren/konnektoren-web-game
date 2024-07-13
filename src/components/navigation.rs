use crate::components::SocialLinks;
use crate::i18n;
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Navigation() -> Html {
    html! {
        <div class="navigation-wrapper">
            <div class="navigation">
                <nav>
                    <Link<Route> to={Route::Map}>{ i18n!("Map") }</Link<Route>>
                    <Link<Route> to={Route::Home}>{ i18n!("Home") }</Link<Route>>
                    <Link<Route> to={Route::About}>{ i18n!("About") }</Link<Route>>
                </nav>
                <div class="tour-social-links">
                    <SocialLinks telegram="https://t.me/KonnektorenHelpBot" web="https://info.konnektoren.help" />
                </div>
            </div>
        </div>
    }
}

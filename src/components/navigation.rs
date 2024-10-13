use crate::Route;
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Navigation() -> Html {
    let i18n = use_i18n();
    html! {
        <div class="navigation-wrapper">
            <div class="navigation">
                <nav>
                    <Link<Route> to={Route::Map}>{ i18n.t("Map") }</Link<Route>>
                    <Link<Route> to={Route::Home}>{ i18n.t("Home") }</Link<Route>>
                    <Link<Route> to={Route::About}>{ i18n.t("About") }</Link<Route>>
                </nav>
            </div>
        </div>
    }
}

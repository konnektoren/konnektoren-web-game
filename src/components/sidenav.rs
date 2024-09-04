use crate::components::social_links::SocialLinks;
use crate::components::ThemeToggle;
use crate::components::{Badge, Logo};
use crate::route::Route;
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Sidenav)]
pub fn sidenav() -> Html {
    let i18n = use_i18n();
    let is_open = use_state(|| false);

    let toggle_sidenav = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let sidenav_class = if *is_open { "sidenav open" } else { "sidenav" };

    let badge = if *is_open {
        html! {
            <Badge label="Beta" description={i18n.t("Konnektoren is still in beta and may have some issues.")} />
        }
    } else {
        html! {}
    };

    html! {
        <div class={sidenav_class}>
            {badge}
            <button class={ if *is_open {"closebtn"} else  {"openbtn"}} onclick={toggle_sidenav}>{ if *is_open {"×"} else {"☰"} }</button>
            <nav>
                <Link<Route> to={Route::Home}><Logo img_src={"/assets/images/Finally_Croped_Orange.svg".to_string()} /></Link<Route>>
                <div id="sidenav-profile">
                    <Link<Route> to={Route::Profile}>
                        <i class="fa-solid fa-user"></i><span class="link-text">{ i18n.t("Profile") }</span>
                    </Link<Route>>
                </div>
                <div id="sidenav-map">
                    <Link<Route> to={Route::Map}>
                        <i class="fa-solid fa-map"></i><span class="link-text">{ i18n.t("Map") }</span>
                    </Link<Route>>
                </div>
                <div id="sidenav-leaderboard">
                    <Link<Route> to={Route::Leaderboard}>
                        <i class="fa-solid  fa-ranking-star"></i><span class="link-text">{ i18n.t("Leaderboard") }</span>
                    </Link<Route>>
                </div>
                <div id="sidenav-about">
                    <Link<Route> to={Route::About}>
                        <i class="fa-solid fa-info-circle"></i><span class="link-text">{ i18n.t("About") }</span>
                    </Link<Route>>
                </div>
                <div id="sidenav-settings">
                    <Link<Route> to={Route::Settings}>
                        <i class="fa-solid fa-cog"></i><span class="link-text">{ i18n.t("Settings") }</span>
                    </Link<Route>>
                </div>
            </nav>
            <SocialLinks telegram="https://t.me/KonnektorenHelpBot" web="https://info.konnektoren.help" />
            <ThemeToggle />
        </div>
    }
}

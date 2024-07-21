use crate::components::social_links::SocialLinks;
use crate::components::ThemeToggle;
use crate::route::Route;
use crate::{components::Logo, i18n};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Sidenav)]
pub fn sidenav() -> Html {
    let is_open = use_state(|| false);

    let toggle_sidenav = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let sidenav_class = if *is_open { "sidenav open" } else { "sidenav" };

    html! {
        <>
            <div class={sidenav_class}>
                <button class={ if *is_open {"closebtn"} else  {"openbtn"}} onclick={toggle_sidenav}>{ if *is_open {"×"} else {"☰"} }</button>
                <Link<Route> to={Route::Home}><Logo img_src={"assets/images/Finally_Croped_Orange.svg".to_string()} /></Link<Route>>
                <div>
                    <Link<Route> to={Route::Profile}>
                        <i class="fa-solid fa-user"></i><span class="link-text">{ i18n!("Profile") }</span>
                    </Link<Route>>
                </div>
                <div>
                    <Link<Route> to={Route::Map}>
                        <i class="fa-solid fa-map"></i><span class="link-text">{ i18n!("Map") }</span>
                    </Link<Route>>
                </div>
                <div>
                    <Link<Route> to={Route::Leaderboard}>
                        <i class="fa-solid  fa-ranking-star"></i><span class="link-text">{ i18n!("Leaderboard") }</span>
                    </Link<Route>>
                </div>
                <div>
                    <Link<Route> to={Route::About}>
                        <i class="fa-solid fa-info-circle"></i><span class="link-text">{ i18n!("About") }</span>
                    </Link<Route>>
                </div>
                <SocialLinks telegram="https://t.me/KonnektorenHelpBot" web="https://info.konnektoren.help" />
                <ThemeToggle />
            </div>
        </>
    }
}

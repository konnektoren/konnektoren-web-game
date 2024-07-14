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
                <button class="closebtn" onclick={toggle_sidenav.clone()}>{ "×" }</button>
                <Link<Route> to={Route::Home}><Logo img_src={"/favicon.png".to_string()} /></Link<Route>>
                <Link<Route> to={Route::Profile}>{ i18n!("Profile") }</Link<Route>>
                <Link<Route> to={Route::Map}>{ i18n!("Map") }</Link<Route>>
                <Link<Route> to={Route::About}>{ i18n!("About") }</Link<Route>>
                <SocialLinks telegram="https://t.me/KonnektorenHelpBot" web="https://info.konnektoren.help" />
                <ThemeToggle />
            </div>
            <button class="openbtn" onclick={toggle_sidenav}>{ "☰" }</button>
        </>
    }
}

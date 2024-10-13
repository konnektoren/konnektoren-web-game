use crate::components::social_links::SocialLinks;
use crate::components::{Badge, Logo};
use crate::components::{InboxComponent, ThemeToggle};
use crate::Route;
use konnektoren_yew::i18n::use_i18n;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Sidenav)]
pub fn sidenav() -> Html {
    let i18n = use_i18n();
    let is_open = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let search_query = use_state(String::new);
    let toggle_sidenav = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let handle_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let handle_search_submit = {
        let search_query = search_query.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let query = (*search_query).clone();
            if !query.is_empty() {
                navigator.push(&Route::SearchWithQuery {
                    query: query.clone(),
                });
            }
        })
    };

    let handle_search_click = {
        let search_query = search_query.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let query = (*search_query).clone();
            navigator.push(&Route::SearchWithQuery {
                query: query.clone(),
            });
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

    let search_input = if *is_open {
        html! {
            <form onsubmit={handle_search_submit} class="search-form">
                <input
                    type="text"
                    placeholder={i18n.t("Search")}
                    value={(*search_query).clone()}
                    oninput={handle_search_input}
                />
                <button type="button" onclick={handle_search_click}>
                    <i class="fa-solid fa-search"></i>
                </button>
            </form>
        }
    } else {
        html! {}
    };

    html! {
        <div class={sidenav_class}>
            {badge}
            <button class={ if *is_open {"closebtn"} else  {"openbtn"}} onclick={toggle_sidenav}>{ if *is_open {"×"} else {"☰"} }</button>
            {search_input}
            <nav>
                <Link<Route> to={Route::Home}><Logo img_src={"/assets/images/Finally_Croped_Orange.svg".to_string()} /></Link<Route>>
                <InboxComponent />
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
                <div id="sidenav-challenges">
                    <Link<Route> to={Route::Challenges}>
                        <i class="fa-solid fa-tasks"></i><span class="link-text">{ i18n.t("Challenges") }</span>
                    </Link<Route>>
                </div>
                <div id="sidenav-achievements">
                    <Link<Route> to={Route::Achievements}>
                        <i class="fa-solid fa-trophy"></i><span class="link-text">{ i18n.t("Achievements") }</span>
                    </Link<Route>>
                </div>
                <div id="sidenav-leaderboard">
                    <Link<Route> to={Route::Leaderboard}>
                        <i class="fa-solid  fa-ranking-star"></i><span class="link-text">{ i18n.t("Leaderboard") }</span>
                    </Link<Route>>
                </div>
                <div id="sidenav-marketplace">
                    <Link<Route> to={Route::Marketplace}>
                        <i class="fa-solid fa-store"></i><span class="link-text">{ i18n.t("Marketplace") }</span>
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

use super::menu::NavigationMenu;
use super::types::{NavExtra, NavGroup, NavItem};
use crate::components::{Sidenav, SocialLinks};
use crate::Route;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::model::Design;
use konnektoren_yew::prelude::{use_design, SelectTheme};
use yew::prelude::*;
use yew_router::components::Link;

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    pub groups: Vec<NavGroup>,
}

#[function_component]
pub fn Navigation() -> Html {
    let design = use_design();

    match &*design {
        Design::Desktop => {
            html! {
                <NavigationDesktopDesign />
            }
        }
        Design::Mobile => {
            html! {
                <NavigationMobileDesign />
            }
        }
        _ => html! {
            <NavigationDesktopDesign />
        },
    }
}

#[function_component]
pub fn NavigationDesktopDesign() -> Html {
    let i18n = use_i18n();
    html! {
        <>
        <Sidenav />
        <div class="navigation-wrapper">
            <div class="navigation">
                <nav>
                    <Link<Route> to={Route::Map}>{ i18n.t("Map") }</Link<Route>>
                    <Link<Route> to={Route::Home}>{ i18n.t("Home") }</Link<Route>>
                    <Link<Route> to={Route::About}>{ i18n.t("About") }</Link<Route>>
                </nav>
            </div>
        </div>
        </>
    }
}

#[function_component]
pub fn NavigationMobileDesign() -> Html {
    let nav_groups = vec![
        NavGroup {
            name: "Main",
            icon: "fa-solid fa-house",
            items: vec![
                NavItem {
                    name: "Home",
                    route: Route::Home,
                    icon: "fa-solid fa-home",
                },
                NavItem {
                    name: "Challenges",
                    route: Route::Challenges,
                    icon: "fa-solid fa-tasks",
                },
                NavItem {
                    name: "Map",
                    route: Route::Map,
                    icon: "fa-solid fa-map",
                },
                NavItem {
                    name: "About",
                    route: Route::About,
                    icon: "fa-solid fa-info-circle",
                },
            ],
            extras: None,
        },
        NavGroup {
            name: "Social",
            icon: "fa-solid fa-users",
            items: vec![
                NavItem {
                    name: "Profile",
                    route: Route::Profile,
                    icon: "fa-solid fa-user",
                },
                NavItem {
                    name: "Leaderboard",
                    route: Route::Leaderboard,
                    icon: "fa-solid fa-ranking-star",
                },
                NavItem {
                    name: "Session",
                    route: Route::Session {
                        id: "new".to_string(),
                    },
                    icon: "fa-solid fa-users",
                },
            ],
            extras: None,
        },
        NavGroup {
            name: "Progress",
            icon: "fa-solid fa-chart-line",
            items: vec![NavItem {
                name: "Achievements",
                route: Route::Achievements,
                icon: "fa-solid fa-trophy",
            }],
            extras: None,
        },
        NavGroup {
            name: "Shop",
            icon: "fa-solid fa-store",
            items: vec![NavItem {
                name: "Marketplace",
                route: Route::Marketplace,
                icon: "fa-solid fa-store",
            }],
            extras: None,
        },
        NavGroup {
            name: "Settings",
            icon: "fa-solid fa-cog",
            items: vec![NavItem {
                name: "Settings",
                route: Route::Settings,
                icon: "fa-solid fa-cog",
            }],
            extras: None,
        },
        NavGroup {
            name: "Links",
            icon: "fa-solid fa-link",
            items: vec![],
            extras: Some(vec![
                NavExtra::Component(html! {
                    <SocialLinks
                        bluesky="https://bsky.app/profile/konnektoren.help"
                        telegram="https://t.me/KonnektorenHelpBot"
                        web="https://info.konnektoren.help"
                    />
                }),
                NavExtra::Component(html! {
                    <SelectTheme />
                }),
            ]),
        },
    ];

    html! {
        <header class="navigation-wrapper">
            <NavigationMenu groups={nav_groups} />
        </header>
    }
}

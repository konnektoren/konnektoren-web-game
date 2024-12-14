use super::types::{NavExtra, NavGroup};
use crate::Route;
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    pub groups: Vec<NavGroup>,
}

#[function_component(NavigationMenu)]
pub fn navigation_menu(props: &MenuProps) -> Html {
    let i18n = use_i18n();
    let active_group = use_state(|| None::<usize>);

    let toggle_group = {
        let active_group = active_group.clone();
        move |index: usize| {
            let active_group_clone = active_group.clone();
            Callback::from(move |_| {
                if Some(index) == *active_group_clone {
                    active_group_clone.set(None);
                } else {
                    active_group_clone.set(Some(index));
                }
            })
        }
    };

    html! {
        <>
            if let Some(active_idx) = *active_group {
                <div class="navigation-dropdown" onclick={Callback::from({
                    let active_group = active_group.clone();
                    move |_| active_group.set(None)
                })}>
                    <nav>
                        {
                            props.groups[active_idx].items.iter().map(|item| {
                                html! {
                                    <Link<Route> to={item.route.clone()} >
                                        <i class={item.icon}></i>
                                        <span>{ i18n.t(item.name) }</span>
                                    </Link<Route>>
                                }
                            }).collect::<Html>()
                        }
                        {
                            if let Some(extras) = &props.groups[active_idx].extras {
                                html! {
                                    <div class="nav-extras">
                                        {
                                            extras.iter().map(|extra| {
                                                match extra {
                                                    NavExtra::Component(component) => component.clone(),
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </nav>
                </div>
            }
            <nav class="navigation">
                {
                    props.groups.iter().enumerate().map(|(idx, group)| {
                        let is_active = Some(idx) == *active_group;
                        html! {
                            <button
                                class={if is_active { "nav-group active" } else { "nav-group" }}
                                onclick={toggle_group(idx)}
                            >
                                <i class={group.icon}></i>
                                <span>{ i18n.t(group.name) }</span>
                            </button>
                        }
                    }).collect::<Html>()
                }
            </nav>
        </>
    }
}

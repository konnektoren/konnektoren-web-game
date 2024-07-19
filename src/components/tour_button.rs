use crate::components::Tour;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(TourButton)]
pub fn tour_button(props: &Props) -> Html {
    let show_tour = use_state(|| false);

    let show_tour_button = {
        let default_show = LocalStorage::get(format!("{}-show", props.id))
            .unwrap_or_else(|_| "true".to_string())
            == "true";
        use_state(|| default_show)
    };

    let on_click = {
        let show_tour = show_tour.clone();
        Callback::from(move |_| {
            show_tour.set(true);
        })
    };

    match (*show_tour_button, *show_tour) {
        (true, false) => {
            html! {
                <div class="tour-button">
                    <button onclick={on_click}> { "Start Onboarding Tour" } </button>
                </div>
            }
        }
        (true, true) => {
            html! {
                <div class="tour-welcome">
                    <Tour id={props.id.to_string()} data={include_str!("../assets/main-tour.yml")} />
                </div>
            }
        }
        _ => {
            html! { <></> }
        }
    }
}

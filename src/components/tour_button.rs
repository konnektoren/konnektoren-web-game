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

    let show_tour_button =
        use_state(|| LocalStorage::get::<bool>(format!("{}-show", props.id)).unwrap_or(true));

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
                    <button class="tour-button__btn" onclick={on_click}> { "Start Tour" } </button>
                </div>
            }
        }
        (true, true) => {
            html! {
                <div class="tour-welcome">
                    <Tour
                        id={props.id.clone()}
                        data={include_str!("../assets/main-tour.yml")}
                    />
                </div>
            }
        }
        _ => {
            html! { <></> }
        }
    }
}

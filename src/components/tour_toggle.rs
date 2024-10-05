use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use konnektoren_yew::i18n::use_i18n;

const TOUR_BUTTON_KEY: &str = "main-show";

#[function_component(TourToggle)]
pub fn tour_toggle() -> Html {
    let i18n = use_i18n();
    let tour_button_visible =
        use_state(|| LocalStorage::get::<bool>(TOUR_BUTTON_KEY).unwrap_or(true));

    {
        let tour_button_visible = tour_button_visible.clone();
        use_effect_with((*tour_button_visible).clone(), move |_| {
            LocalStorage::set(TOUR_BUTTON_KEY, *tour_button_visible).unwrap();
            || ()
        });
    }

    let toggle_tour_button = {
        let tour_button_visible = tour_button_visible.clone();
        Callback::from(move |_| {
            tour_button_visible.set(!*tour_button_visible);
        })
    };

    html! {
        <div class="tour-toggle">
            <label>
                <input type="checkbox"
                    checked={*tour_button_visible}
                    onclick={toggle_tour_button.clone()} />
                { i18n.t("Show Tour Button")}
            </label>
        </div>
    }
}

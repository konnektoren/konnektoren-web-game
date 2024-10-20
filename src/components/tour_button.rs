use crate::components::Tour;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::providers::use_settings_repository;
use konnektoren_yew::repository::SETTINGS_STORAGE_KEY;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(TourButton)]
pub fn tour_button(props: &Props) -> Html {
    let i18n = use_i18n();
    let show_tour = use_state(|| false);

    let show_helpers = use_state(|| true);
    let settings_repository = use_settings_repository();

    {
        let show_helpers = show_helpers.clone();
        let settings_repository = settings_repository.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let settings_repository = settings_repository.clone();

                let stored_settings = settings_repository
                    .get_settings(SETTINGS_STORAGE_KEY)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_default();
                show_helpers.set(stored_settings.show_helpers);
            });
        });
    }

    let on_click = {
        let show_tour = show_tour.clone();
        let settings_repository = settings_repository.clone();
        Callback::from(move |_| {
            show_tour.set(true);
            let settings_repository = settings_repository.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let settings_repository = settings_repository.clone();

                let mut stored_settings = settings_repository
                    .get_settings(SETTINGS_STORAGE_KEY)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_default();
                stored_settings.show_helpers = false;
                settings_repository
                    .save_settings(SETTINGS_STORAGE_KEY, &stored_settings)
                    .await
                    .unwrap();
            });
        })
    };

    match (*show_helpers, *show_tour) {
        (true, false) => {
            html! {
                <div class="tour-button">
                    <button class="tour-button__btn" onclick={on_click}> { i18n.t("Start Tour") } </button>
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

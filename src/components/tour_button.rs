use crate::components::Tour;
use crate::repository::{LocalStorage, Repository, SettingsRepository, SETTINGS_STORAGE_KEY};
use konnektoren_yew::i18n::use_i18n;
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

    {
        let show_helpers = show_helpers.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let settings_repository =
                    SettingsRepository::new(LocalStorage::new(Some(SETTINGS_STORAGE_KEY)));

                let stored_settings = settings_repository
                    .get(SETTINGS_STORAGE_KEY)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_default();
                show_helpers.set(stored_settings.show_helpers);
            });
        });
    }

    let on_click = {
        let show_tour = show_tour.clone();
        Callback::from(move |_| {
            show_tour.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let settings_repository =
                    SettingsRepository::new(LocalStorage::new(Some(SETTINGS_STORAGE_KEY)));

                let mut stored_settings = settings_repository
                    .get(SETTINGS_STORAGE_KEY)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_default();
                stored_settings.show_helpers = false;
                settings_repository
                    .save(SETTINGS_STORAGE_KEY, &stored_settings)
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

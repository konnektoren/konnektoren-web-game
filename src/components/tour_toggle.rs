use crate::repository::{LocalStorage, Repository, SettingsRepository, SETTINGS_STORAGE_KEY};
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;

#[function_component(TourToggle)]
pub fn tour_toggle() -> Html {
    let i18n = use_i18n();
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

    let toggle_show_helpers = {
        let show_helpers = show_helpers.clone();
        Callback::from(move |_| {
            let new_show_helpers = !*show_helpers;
            show_helpers.set(new_show_helpers);
            wasm_bindgen_futures::spawn_local(async move {
                let settings_repository =
                    SettingsRepository::new(LocalStorage::new(Some(SETTINGS_STORAGE_KEY)));

                let mut stored_settings = settings_repository
                    .get(SETTINGS_STORAGE_KEY)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_default();
                stored_settings.show_helpers = new_show_helpers;
                settings_repository
                    .save(SETTINGS_STORAGE_KEY, &stored_settings)
                    .await
                    .unwrap();
            });
        })
    };

    html! {
        <div class="tour-toggle">
            <label>
                <input type="checkbox"
                    checked={*show_helpers}
                    onclick={toggle_show_helpers.clone()} />
                { i18n.t("Show Tour Button")}
            </label>
        </div>
    }
}

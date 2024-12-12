use konnektoren_yew::providers::use_settings_repository;
use konnektoren_yew::repository::SETTINGS_STORAGE_KEY;
use yew::prelude::*;

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let theme = use_state(|| String::from("light"));
    let settings_repository = use_settings_repository();

    let update_theme_class = |new_theme: &str| {
        if let Some(body) = gloo::utils::document().body() {
            let current_classes: Vec<String> = body
                .class_name()
                .split_whitespace()
                .filter(|class| !class.starts_with("theme-"))
                .map(String::from)
                .collect();

            let mut classes = current_classes;
            classes.push(format!("theme-{}", new_theme));

            body.set_class_name(&classes.join(" "));
        }
    };

    {
        let theme = theme.clone();
        let settings_repository = settings_repository.clone();

        use_effect_with((), move |_| {
            update_theme_class(&theme);
            let settings_repository = settings_repository.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let stored_settings = settings_repository
                    .get_settings(SETTINGS_STORAGE_KEY)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_default();
                theme.set(stored_settings.theme);
            });
        });
    }

    {
        let theme = theme.clone();
        use_effect(move || {
            update_theme_class(&theme);
            || ()
        });
    }

    let toggle_theme = {
        let theme = theme.clone();
        let settings_repository = settings_repository.clone();
        Callback::from(move |_| {
            let new_theme = if *theme == "light" {
                "dark".to_string()
            } else if *theme == "dark" {
                "star".to_string()
            } else if *theme == "star" {
                "light".to_string()
            } else {
                "light".to_string()
            };
            theme.set(new_theme.clone());
            let settings_repository = settings_repository.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut stored_settings = settings_repository
                    .get_settings(SETTINGS_STORAGE_KEY)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_default();
                stored_settings.theme = new_theme;
                settings_repository
                    .save_settings(SETTINGS_STORAGE_KEY, &stored_settings)
                    .await
                    .unwrap();
            });
        })
    };

    html! {
        <button class="theme-toggle" onclick={toggle_theme}>
            {
                if *theme == "light" {
                    html! { <i class="fas fa-moon"></i> }
                } else if *theme == "dark" {
                    html! { <i class="fas fa-star"></i> }
                } else if *theme == "star" {
                    html! { <i class="fas fa-sun"></i> }
                } else {
                    html! { <i class="fas fa-sun"></i> }
                }
            }
        </button>
    }
}

use crate::components::{ThemeToggle, TourToggle};
use konnektoren_yew::components::SelectLanguage;
use konnektoren_yew::prelude::SettingsComponent;
use yew::prelude::*;

#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    html! {
        <div class="settings-page">
            <SettingsComponent />
            <TourToggle />
            <ThemeToggle />
            <SelectLanguage />
        </div>
    }
}

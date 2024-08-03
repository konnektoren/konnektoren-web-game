use crate::components::Map;
use gloo::utils::document;
use konnektoren_yew::components::{MusicComponent, ProfilePointsComponent};
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::Link;
use crate::route::Route;

#[function_component(MapPage)]
pub fn map_page() -> Html {
    let i18n = use_i18n();
    let title = format!("Konnektoren - {}", i18n.t("Map"));
    use_effect(move || {
        document().set_title(&title);
        || ()
    });
    html! {
        <div class="map-page">
            <MusicComponent url="music/background_main.wav" />
            <Link<Route> to={Route::Profile}><ProfilePointsComponent /></Link<Route>>
            <h1>{ i18n.t("Map") }</h1>
            <Map />
        </div>
    }
}

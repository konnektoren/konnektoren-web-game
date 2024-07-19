use crate::{components::Map, i18n};
use konnektoren_yew::components::{MusicComponent, ProfilePointsComponent};
use yew::prelude::*;

#[function_component(MapPage)]
pub fn map_page() -> Html {
    html! {
        <div class="map-page">
            <MusicComponent url="music/background_main.wav" />
            <ProfilePointsComponent />
            <h1>{ i18n!("Map") }</h1>
            <Map />
        </div>
    }
}

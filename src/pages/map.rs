use crate::components::Tour;
use crate::{components::Map, i18n};
use konnektoren_yew::components::{MusicComponent, ProfilePointsComponent};
use yew::prelude::*;

#[function_component(MapPage)]
pub fn map_page() -> Html {
    html! {
        <div class="map-page">
            <MusicComponent url="music/background_main.wav" />
            <Tour id="map" data={include_str!("../assets/map-tour.yml")} />
            <ProfilePointsComponent />
            <h1>{ i18n!("Map") }</h1>
            <Map />
        </div>
    }
}

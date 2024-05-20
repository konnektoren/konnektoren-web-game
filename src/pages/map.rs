use crate::components::{Footer, Map};
use yew::prelude::*;

#[function_component(MapPage)]
pub fn map_page() -> Html {
    html! {
        <div class="map-page">
            <h1>{ "Map" }</h1>

            <Map />

            <Footer />
        </div>
    }
}

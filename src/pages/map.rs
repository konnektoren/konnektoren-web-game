use crate::{
    components::{Footer, Map},
    i18n,
};
use yew::prelude::*;

#[function_component(MapPage)]
pub fn map_page() -> Html {
    html! {
        <div class="map-page">
            <h1>{ i18n!("Map") }</h1>

            <Map />

            <Footer />
        </div>
    }
}

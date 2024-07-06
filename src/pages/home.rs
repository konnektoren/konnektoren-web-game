use yew::prelude::*;
use crate::components::Tour;

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <div>
            <Tour />
            <h1>{ "Coming soon!" }</h1>
        </div>
    }
}

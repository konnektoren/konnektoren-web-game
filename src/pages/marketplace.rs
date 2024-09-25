use yew::prelude::*;

#[function_component(MarketplacePage)]
pub fn marketplace_page() -> Html {
    html! {
        <div class="marketplace-page">
            <h1>{"Marketplace"}</h1>
            <p>{"Welcome to the marketplace!"}</p>
        </div>
    }
}

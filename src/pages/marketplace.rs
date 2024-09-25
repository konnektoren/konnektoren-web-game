use konnektoren_core::marketplace::ProductCatalog;
use konnektoren_yew::components::ProductCatalogComponent;
use yew::prelude::*;

#[function_component(MarketplacePage)]
pub fn marketplace_page() -> Html {
    let product_catalog =
        ProductCatalog::from_yaml(include_str!("../assets/product_catalog.yml")).unwrap();

    html! {
        <div class="marketplace-page">
            <h1>{"Marketplace"}</h1>
            <ProductCatalogComponent product_catalog={product_catalog} />
        </div>
    }
}

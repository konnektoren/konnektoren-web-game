use crate::model::product_repository::ProductRepository;
use konnektoren_core::marketplace::ProductCatalog;
use konnektoren_yew::components::ProductCatalogComponent;
use yew::prelude::*;

#[function_component(MarketplacePage)]
pub fn marketplace_page() -> Html {
    let product_catalog =
        ProductCatalog::from_yaml(include_str!("../assets/product_catalog.yml")).unwrap();

    let on_select = Callback::from(|product| {
        log::info!("Selected product: {:?}", product);
        ProductRepository::new().store(product);
    });
    html! {
        <div class="marketplace-page">
            <h1>{"Marketplace"}</h1>
            <ProductCatalogComponent {product_catalog} {on_select} />
        </div>
    }
}

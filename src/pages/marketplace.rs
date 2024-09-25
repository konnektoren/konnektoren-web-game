use crate::model::product_repository::ProductRepository;
use konnektoren_core::marketplace::ProductCatalog;
use konnektoren_yew::components::ProductCatalogComponent;
use yew::prelude::*;

#[function_component(MarketplacePage)]
pub fn marketplace_page() -> Html {
    let product_catalog = use_state(|| product_catalog_without_buyed());
    let on_select = {
        let product_catalog = product_catalog.clone();
        Callback::from(move |product| {
            let product_catalog = product_catalog.clone();
            log::info!("Selected product: {:?}", product);
            ProductRepository::new().store(product);
            product_catalog.set(product_catalog_without_buyed());
        })
    };

    let product_catalog = (*product_catalog).clone();

    html! {
        <div class="marketplace-page">
            <h1>{"Marketplace"}</h1>
            <ProductCatalogComponent {product_catalog} {on_select} />
        </div>
    }
}

fn product_catalog_without_buyed() -> ProductCatalog {
    let mut product_catalog =
        ProductCatalog::from_yaml(include_str!("../assets/product_catalog.yml")).unwrap();
    let game_path = ProductRepository::new().fetch_custom_level();

    let buyed_products = game_path
        .unwrap_or_default()
        .challenges
        .iter()
        .map(|challenge| challenge.id.clone())
        .collect::<Vec<String>>();

    product_catalog.products.retain(|product| {
        !buyed_products.contains(&product.id.as_ref().unwrap_or(&"".to_string()))
    });
    product_catalog
}

use crate::model::product_repository::ProductRepository;
use crate::pages::marketplace::search_product_catalog::SearchProductCatalog;
use konnektoren_core::marketplace::{Product, ProductCatalog};
use konnektoren_yew::components::ProductCatalogComponent;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(MarketplacePage)]
pub fn marketplace_page() -> Html {
    let product_catalog = use_state(|| product_catalog_without_buyed());
    let search_query = use_state(|| "".to_string());
    let suggestion = use_state(|| "".to_string());

    let search_product_catalog = use_state(|| SearchProductCatalog::new(&*product_catalog));

    let filtered_product_catalog = match &**search_query {
        "" => (*product_catalog).clone(),
        _ => search_product_catalog.filtered(&*search_query),
    };

    let on_select = {
        let product_catalog = product_catalog.clone();
        let search_product_catalog = search_product_catalog.clone();
        Callback::from(move |product: Product| {
            let product_catalog = product_catalog.clone();
            log::info!("Selected product: {:?}", product);
            ProductRepository::new().store(product);
            product_catalog.set(product_catalog_without_buyed());
            search_product_catalog.set(SearchProductCatalog::new(&product_catalog_without_buyed()));
        })
    };

    let on_search_query_change = {
        let search_query = search_query.clone();
        let suggestion = suggestion.clone();
        let search_product_catalog = search_product_catalog.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            // Update the search query
            search_query.set(value.clone());

            // Get a suggestion based on the current input
            if let Some(suggested) = search_product_catalog.get_suggestion(&value) {
                suggestion.set(suggested);
            } else {
                suggestion.set("".to_string());
            }
        })
    };

    let on_suggestion_click = {
        let suggestion = suggestion.clone();
        let search_query = search_query.clone();
        Callback::from(move |_| {
            search_query.set((*suggestion).clone());
            suggestion.set("".to_string());
        })
    };

    html! {
        <div class="marketplace-page">
            <h1>{"Marketplace"}</h1>

            <div class="search-container">
            <input
                id="search-query"
                type="text"
                value={(*search_query).clone()}
                oninput={on_search_query_change}
                placeholder="Search products..."
            />
            if !(*suggestion).is_empty() && *suggestion != *search_query {
                <div class="suggestion" onclick={on_suggestion_click}>
                    {format!("Did you mean: {}", *suggestion)}
                </div>
            }
            </div>
            <ProductCatalogComponent product_catalog={filtered_product_catalog} {on_select} />
        </div>
    }
}

fn product_catalog_without_buyed() -> ProductCatalog {
    let mut product_catalog =
        ProductCatalog::from_yaml(include_str!("../../assets/product_catalog.yml")).unwrap();
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

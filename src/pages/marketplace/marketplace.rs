use crate::components::CheckoutComponent;
use crate::model::product_repository::ProductRepository;
use crate::pages::marketplace::search_product_catalog::SearchProductCatalog;
use konnektoren_core::marketplace::{Product, ProductCatalog};
use konnektoren_core::prelude::Cart;
use konnektoren_yew::components::{CartBadgeComponent, ProductCatalogComponent};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub enum MarketplacePageState {
    Catalog {
        cart: Cart,
        product_catalog: ProductCatalog,
        search_query: String,
        suggestion: String,
    },
    Cart(Cart),
}

#[function_component(MarketplacePage)]
pub fn marketplace_page() -> Html {
    let state = use_state(|| MarketplacePageState::Catalog {
        cart: Cart::default(),
        product_catalog: product_catalog_without_buyed(),
        search_query: String::new(),
        suggestion: String::new(),
    });

    let on_select = {
        let state = state.clone();
        Callback::from(move |product: Product| {
            state.set(match &*state {
                MarketplacePageState::Catalog {
                    cart,
                    product_catalog,
                    ..
                } => {
                    let mut product_catalog = product_catalog.clone();
                    product_catalog.products.retain(|p| p.id != product.id);
                    let mut cart = cart.clone();
                    cart.add_product(product.clone());
                    ProductRepository::new().store(product);
                    MarketplacePageState::Catalog {
                        cart,
                        product_catalog: product_catalog.clone(),
                        search_query: String::new(),
                        suggestion: String::new(),
                    }
                }
                MarketplacePageState::Cart(cart) => {
                    let mut cart = cart.clone();
                    cart.add_product(product.clone());
                    ProductRepository::new().store(product);
                    MarketplacePageState::Cart(cart)
                }
            });
        })
    };

    let on_search_query_change = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            state.set(match &*state {
                MarketplacePageState::Catalog {
                    cart,
                    product_catalog,
                    ..
                } => {
                    let search_product_catalog = SearchProductCatalog::new(&product_catalog);
                    let suggestion = search_product_catalog
                        .get_suggestion(&value)
                        .unwrap_or_default();

                    MarketplacePageState::Catalog {
                        cart: cart.clone(),
                        product_catalog: product_catalog.clone(),
                        search_query: value,
                        suggestion,
                    }
                }
                _ => (&*state).clone(),
            });
        })
    };

    let on_suggestion_click = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(match &*state {
                MarketplacePageState::Catalog {
                    cart,
                    product_catalog,
                    suggestion,
                    ..
                } => MarketplacePageState::Catalog {
                    cart: cart.clone(),
                    product_catalog: product_catalog.clone(),
                    search_query: suggestion.clone(),
                    suggestion: String::new(),
                },
                _ => (&*state).clone(),
            });
        })
    };

    let on_cart_click = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(match &*state {
                MarketplacePageState::Catalog { cart, .. } => {
                    MarketplacePageState::Cart(cart.clone())
                }
                MarketplacePageState::Cart(cart) => MarketplacePageState::Catalog {
                    cart: cart.clone(),
                    product_catalog: product_catalog_without_cart(
                        &product_catalog_without_buyed(),
                        &cart,
                    ),
                    search_query: String::new(),
                    suggestion: String::new(),
                },
            });
        })
    };

    let on_product_remove = {
        let state = state.clone();
        Callback::from(move |product: Product| {
            state.set(match &*state {
                MarketplacePageState::Cart(cart) => {
                    let mut cart = cart.clone();
                    cart.remove_product(&product.id.unwrap_or_default());
                    MarketplacePageState::Cart(cart.clone())
                }
                _ => (&*state).clone(),
            });
        })
    };

    let on_checkout_complete = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(match &*state {
                MarketplacePageState::Catalog { cart, .. } => {
                    MarketplacePageState::Cart(cart.clone())
                }
                MarketplacePageState::Cart(cart) => MarketplacePageState::Catalog {
                    cart: Cart::default(),
                    product_catalog: product_catalog_without_buyed(),
                    search_query: String::new(),
                    suggestion: String::new(),
                },
            });
        })
    };

    let cart = match &*state {
        MarketplacePageState::Cart(cart) => cart.clone(),
        MarketplacePageState::Catalog { cart, .. } => cart.clone(),
    };

    html! {
        <div class="marketplace-page">
            <CartBadgeComponent
               cart={cart}
               on_click={on_cart_click.clone()}
           />
            <h1>{"Marketplace"}</h1>
            {
                match &*state {
                    MarketplacePageState::Catalog { product_catalog, search_query, suggestion, .. } => render_product_catalog(
                        product_catalog,
                        search_query,
                        suggestion,
                        on_search_query_change.clone(),
                        on_suggestion_click.clone(),
                        on_select.clone(),
                    ),
                    MarketplacePageState::Cart(cart) => render_cart(
                        cart, on_product_remove.clone(), on_checkout_complete.clone()
                    ),
                }
            }
        </div>
    }
}

fn render_product_catalog(
    product_catalog: &ProductCatalog,
    search_query: &str,
    suggestion: &str,
    on_search_query_change: Callback<InputEvent>,
    on_suggestion_click: Callback<()>,
    on_select: Callback<Product>,
) -> Html {
    let on_suggestion_click = on_suggestion_click.reform(|_| ());
    html! {
        <>
            <div class="search-container">
                <input
                    id="search-query"
                    type="text"
                    value={search_query.to_string()}
                    oninput={on_search_query_change}
                    placeholder="Search products..."
                />
                if !suggestion.is_empty() && suggestion != search_query {
                    <div class="suggestion" onclick={on_suggestion_click}>
                        {format!("Did you mean: {}", suggestion)}
                    </div>
                }
            </div>
            <ProductCatalogComponent product_catalog={product_catalog.clone()} {on_select} />
        </>
    }
}

fn render_cart(cart: &Cart, on_select: Callback<Product>, on_complete: Callback<()>) -> Html {
    html! {
        <CheckoutComponent cart={cart.clone()} {on_select} {on_complete} />
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

fn product_catalog_without_cart(product_catalog: &ProductCatalog, cart: &Cart) -> ProductCatalog {
    let mut product_catalog = product_catalog.clone();
    for product in cart.products.iter() {
        product_catalog.products.retain(|p| p.id != product.id);
    }
    product_catalog
}

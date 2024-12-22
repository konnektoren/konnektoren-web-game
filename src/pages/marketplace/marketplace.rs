use crate::components::CheckoutComponent;
use crate::model::product_repository::ProductRepository;
use crate::model::ChallengeTypesRepository;
use crate::pages::marketplace::search_product_catalog::SearchProductCatalog;
use crate::Route;
use konnektoren_core::marketplace::{Product, ProductCatalog};
use konnektoren_core::prelude::Cart;
use konnektoren_core::session::Session;
use konnektoren_yew::components::{CartBadgeComponent, ProductCatalogComponent};
use konnektoren_yew::providers::use_session;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

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
                    MarketplacePageState::Catalog {
                        cart,
                        product_catalog: product_catalog.clone(),
                        search_query: String::new(),
                        suggestion: String::new(),
                    }
                }
                MarketplacePageState::Cart(_) => (*state).clone(),
            });
        })
    };

    let on_tag = {
        let state = state.clone();
        Callback::from(move |tag: String| {
            state.set(match &*state {
                MarketplacePageState::Catalog {
                    cart,
                    product_catalog,
                    ..
                } => MarketplacePageState::Catalog {
                    cart: cart.clone(),
                    product_catalog: product_catalog.clone(),
                    search_query: tag,
                    suggestion: String::new(),
                },
                _ => (*state).clone(),
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
                    let search_product_catalog = SearchProductCatalog::new(product_catalog);
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
                _ => (*state).clone(),
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
                _ => (*state).clone(),
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
                        cart,
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
                _ => (*state).clone(),
            });
        })
    };

    let on_checkout_complete = {
        let navigator = use_navigator().unwrap();
        let session = use_session();
        Callback::from(move |new_session: Option<Session>| {
            let session = session.clone();
            let mut new_session = new_session.unwrap_or((*session).clone());
            let last_game_path_index = new_session.game_state.game.game_paths.len() - 1;
            new_session.game_state.current_game_path = last_game_path_index;
            session.set(new_session.clone());
            navigator.push(&Route::Challenges);
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
                        on_tag.clone(),
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
    on_tag: Callback<String>,
) -> Html {
    let on_suggestion_click = on_suggestion_click.reform(|_| ());
    let product_catalog = {
        match search_query {
            "" => (*product_catalog).clone(),
            _ => {
                let search_product_catalog = SearchProductCatalog::new(product_catalog);
                search_product_catalog.filtered(search_query)
            }
        }
    };
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
            <ProductCatalogComponent product_catalog={product_catalog.clone()} {on_select} {on_tag} />
        </>
    }
}

fn render_cart(
    cart: &Cart,
    on_select: Callback<Product>,
    on_complete: Callback<Option<Session>>,
) -> Html {
    html! {
        <CheckoutComponent cart={cart.clone()} {on_select} {on_complete} />
    }
}

fn product_catalog_without_buyed() -> ProductCatalog {
    let mut product_catalog =
        ProductCatalog::from_yaml(include_str!("../../assets/product_catalog.yml")).unwrap();
    let game_path = ProductRepository::new().fetch_custom_level();

    let buyed_challenge_types = ChallengeTypesRepository::new()
        .fetch_challenges()
        .unwrap_or_default();

    let buyed_products = game_path
        .unwrap_or_default()
        .challenges
        .iter()
        .map(|challenge| challenge.id.clone())
        .collect::<Vec<String>>();

    product_catalog.products.retain(|product| {
        !buyed_products.contains(product.id.as_ref().unwrap_or(&"".to_string()))
            && !buyed_challenge_types
                .challenges
                .iter()
                .any(|challenge_type| {
                    challenge_type.id() == product.id.as_ref().unwrap_or(&"".to_string())
                })
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

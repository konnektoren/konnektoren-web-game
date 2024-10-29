use crate::components::PaymentComponent;
use crate::model::{ChallengeTypesRepository, ProductRepository};
use konnektoren_core::marketplace::{Cart, CheckoutState, Product};
use konnektoren_core::session::Session;
use konnektoren_yew::components::ShoppingCartComponent;
use konnektoren_yew::prelude::use_session_repository;
use konnektoren_yew::repository::SESSION_STORAGE_KEY;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct CheckoutProps {
    pub cart: Cart,
    pub on_select: Callback<Product>,
    pub on_complete: Callback<Option<Session>>,
}

#[function_component(CheckoutComponent)]
pub fn checkout_component(props: &CheckoutProps) -> Html {
    let cart = props.cart.clone();
    let on_select = props.on_select.clone();
    let on_complete = props.on_complete.clone();

    let session_repository = use_session_repository();

    let state = use_state(|| CheckoutState::new(props.cart.clone()));

    {
        let cart = cart.clone();
        let state = state.clone();
        use_effect_with(state.clone(), move |state| {
            state.set(CheckoutState::new(cart.clone()));
            || {}
        });
    }

    let on_success = {
        let cart = cart.clone();
        let on_complete = on_complete.clone();
        let session_repository = session_repository.clone();
        Callback::from(move |_| {
            let session_repository = session_repository.clone();
            let cart = cart.clone();
            let on_complete = on_complete.clone();
            wasm_bindgen_futures::spawn_local(async move {
                for product in cart.products.iter() {
                    let _ = ProductRepository::new()
                        .store(product.clone(), &*session_repository)
                        .await;
                    let _ = ChallengeTypesRepository::new().store(product.clone()).await;
                }
                let new_session = session_repository
                    .get_session(SESSION_STORAGE_KEY)
                    .await
                    .unwrap();
                on_complete.emit(new_session)
            });
        })
    };

    let on_click = {
        let on_success = on_success.clone();
        Callback::from(move |_| on_success.emit(()))
    };

    let price = cart.total_price();

    let payment_component = match price {
        0.0 => html! {},
        _ => html! { <PaymentComponent {price} on_success={on_success.clone()} /> },
    };

    let checkout_component = match price {
        0.0 => html! {
            <button class="checkout-button" onclick={on_click}>
                <span class="button-text">{ "Complete Purchase" }</span>
                <i class="fas fa-arrow-right"></i>
            </button>
        },
        _ => html! {},
    };

    html! {
        <div class="checkout">
            <h2 class="checkout-title">{ "Checkout" }</h2>
            <div class="shopping-cart-container">
                <ShoppingCartComponent cart={props.cart.clone()} on_select={Some(on_select)} />
            </div>
            { payment_component }
            <div class="checkout-summary">
                <div class="price">{ format!("Total: ${:.2}", price) }</div>
                { checkout_component }
            </div>
        </div>
    }
}

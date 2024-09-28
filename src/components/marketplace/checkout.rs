use crate::components::PaymentComponent;
use crate::model::product_repository::ProductRepository;
use konnektoren_core::marketplace::{Cart, CheckoutState, Product};
use konnektoren_yew::components::ShoppingCartComponent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct CheckoutProps {
    pub cart: Cart,
    pub on_select: Callback<Product>,
    pub on_complete: Callback<()>,
}

#[function_component(CheckoutComponent)]
pub fn checkout_component(props: &CheckoutProps) -> Html {
    let cart = props.cart.clone();
    let on_select = props.on_select.clone();
    let on_complete = props.on_complete.clone();

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
        Callback::from(move |_| {
            let cart = cart.clone();
            let on_complete = on_complete.clone();
            wasm_bindgen_futures::spawn_local(async move {
                for product in cart.products.iter() {
                    let _ = ProductRepository::new().store(product.clone()).await;
                }
                on_complete.emit(())
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

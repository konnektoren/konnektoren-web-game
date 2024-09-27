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

    let on_click = {
        let cart = cart.clone();
        let on_complete = on_complete.clone();
        Callback::from(move |_| {
            cart.products.iter().for_each(|product| {
                ProductRepository::new().store(product.clone());
            });
            on_complete.emit(())
        })
    };

    html! {
        <div class="checkout">
        <ShoppingCartComponent cart={props.cart.clone()} on_select={Some(on_select)} />
        <button onclick={on_click}> { "Checkout" } </button>
        </div>
    }
}

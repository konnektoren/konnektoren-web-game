use konnektoren_yew::prelude::{payTonWallet, TonWalletComponent};
use yew::prelude::*;

const TONCONNECT_MANIFEST_URL: &str = "https://konnektoren.help/assets/tonconnect-manifest.json";
const PAYMENT_ADDRESS: &str = "0:5ca1f07c7d67fd26816a731377b6404e857265761676626a4bd6fda652293119";

#[derive(Properties, Clone, PartialEq)]
pub struct PaymentProps {
    pub price: f64,
}

#[function_component(PaymentComponent)]
pub fn payment_component(props: &PaymentProps) -> Html {
    let address = use_state(|| None::<String>);
    let price = props.price;

    let on_ton_wallet_connect = {
        let address = address.clone();
        Callback::from(move |addr: String| {
            address.set(Some(addr));
        })
    };

    let pay_with_ton_wallet: Html = match &*address {
        Some(addr) => {
            let price = price;
            let nano_ton = (1_000_000_000.0 * price) as u64;
            let on_pay = {
                Callback::from(move |_| {
                    let addr = PAYMENT_ADDRESS.to_string();
                    let nano_ton = nano_ton;
                    wasm_bindgen_futures::spawn_local(async move {
                        log::info!("Payment to {} of {} nanoTON", addr, nano_ton);
                        let result = payTonWallet(addr.clone(), nano_ton).await;
                        match result {
                            Ok(result) => log::info!("Payment successful: {:?}", result),
                            Err(e) => log::error!("Payment error: {:?}", e),
                        }
                    });
                })
            };
            html! {
                <div>
                    <p>{"Payment address: "}{addr}</p>
                    <button onclick={on_pay}>{"Pay with TON"}</button>
                </div>
            }
        }
        None => html! { <div></div> },
    };

    html! {
        <div>
            <h2>{"Payment"}</h2>
            <p>{"Please enter your payment information"}</p>
            <TonWalletComponent manifest_url={TONCONNECT_MANIFEST_URL} on_connect={on_ton_wallet_connect} />
            { pay_with_ton_wallet }
        </div>
    }
}

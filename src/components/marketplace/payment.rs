use gloo::net::http::Request;
use konnektoren_yew::prelude::{payTonWallet, TonWalletComponent};
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const TONCONNECT_MANIFEST_URL: &str = "https://konnektoren.help/assets/tonconnect-manifest.json";
const PAYMENT_ADDRESS: &str = "0:5ca1f07c7d67fd26816a731377b6404e857265761676626a4bd6fda652293119";
const TON_API_URL: &str = "https://testnet.tonapi.io/v2";

#[derive(Properties, Clone, PartialEq)]
pub struct PaymentProps {
    pub price: f64,
    pub on_success: Callback<()>,
}

#[function_component(PaymentComponent)]
pub fn payment_component(props: &PaymentProps) -> Html {
    let address = use_state(|| None::<String>);
    let ton_price = use_state(|| None::<f64>);
    let price = props.price;

    {
        let ton_price = ton_price.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match fetch_ton_price().await {
                    Ok(price) => ton_price.set(Some(price)),
                    Err(e) => log::error!("Failed to fetch TON price: {:?}", e),
                }
            });
            || ()
        });
    }

    let on_ton_wallet_connect = {
        let address = address.clone();
        Callback::from(move |addr: String| {
            address.set(Some(addr));
        })
    };

    let pay_with_ton_wallet: Html = match (&*address, &*ton_price) {
        (Some(addr), Some(ton_usd_price)) => {
            let on_success = props.on_success.clone();
            let ton_amount = price / ton_usd_price;
            let nano_ton = (1_000_000_000.0 * ton_amount) as u64;
            let on_pay = {
                Callback::from(move |_| {
                    let on_success = on_success.clone();
                    let addr = PAYMENT_ADDRESS.to_string();
                    let nano_ton = nano_ton;
                    spawn_local(async move {
                        log::info!("Payment to {} of {} nanoTON", addr, nano_ton);
                        let result = payTonWallet(addr.clone(), nano_ton).await;
                        match result {
                            Ok(result) => {
                                log::info!("Payment successful: {:?}", result);
                                on_success.emit(());
                            }
                            Err(e) => log::error!("Payment error: {:?}", e),
                        }
                    });
                })
            };

            html! {
                <div>
                    <button onclick={on_pay}>{format!("Pay with {:.4} TON", ton_amount)}</button>
                </div>
            }
        }
        _ => html! { <div>{"Loading..."}</div> },
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

async fn fetch_ton_price() -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("{}/rates?tokens=ton&currencies=usd", TON_API_URL);
    let resp = Request::get(&url).send().await?;
    let json: Value = resp.json().await?;

    let usd_price = json["rates"]["TON"]["prices"]["USD"]
        .as_f64()
        .ok_or("Failed to parse TON price")?;

    Ok(usd_price)
}

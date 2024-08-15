use yew::prelude::*;
use gloo::net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, SubmitEvent};
use serde::Serialize;

#[derive(Serialize)]
struct Subscription {
    plan: String,
    price: f32,
}

#[function_component(PaymentPage)]
pub fn payment_page() -> Html {
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let target = event.target().unwrap();
        let form = target.unchecked_into::<HtmlElement>();

        let plan_input = form.query_selector("input[name='plan']").unwrap().unwrap();
        let price_input = form.query_selector("input[name='price']").unwrap().unwrap();

        let plan = plan_input.unchecked_into::<HtmlInputElement>().value();
        let price = price_input.unchecked_into::<HtmlInputElement>().value();

        let subscription = Subscription {
            plan,
            price: price.parse::<f32>().unwrap(),
        };

        wasm_bindgen_futures::spawn_local(async move {
            let request = Request::post("http://127.0.0.1:8080/subscribe")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&subscription).unwrap())
                .expect("Failed to build request");

            match request.send().await {
                Ok(response) => {
                    let result: String = response.text().await.unwrap();
                    gloo::dialogs::alert(&result);
                }
                Err(err) => gloo::dialogs::alert(&format!("Error: {}", err)),
            }
        });
    });

    html! {
        <div class="container">
            <h1>{ "Choose Your Subscription Plan" }</h1>
            <div class="plans">
                <div class="plan">
                    <h2 style="color: lightblue;">{ "Basic Plan" }</h2>
                    <p>{ "$9.99/month" }</p>
                    <form onsubmit={onsubmit.clone()}>
                        <input type="hidden" name="plan" value="Basic Plan"/>
                        <input type="hidden" name="price" value="9.99"/>
                        <button type="submit" style="background: linear-gradient(orange, darkorange);">{ "Subscribe" }</button>
                    </form>
                </div>
                <div class="plan">
                    <h2 style="color: lightblue;">{ "Standard Plan" }</h2>
                    <p>{ "$19.99/month" }</p>
                    <form onsubmit={onsubmit.clone()}>
                        <input type="hidden" name="plan" value="Standard Plan"/>
                        <input type="hidden" name="price" value="19.99"/>
                        <button type="submit" style="background: linear-gradient(orange, darkorange);">{ "Subscribe" }</button>
                    </form>
                </div>
                <div class="plan">
                    <h2 style="color: lightblue;">{ "Premium Plan" }</h2>
                    <p>{ "$29.99/month" }</p>
                    <form onsubmit={onsubmit.clone()}>
                        <input type="hidden" name="plan" value="Premium Plan"/>
                        <input type="hidden" name="price" value="29.99"/>
                        <button type="submit" style="background: linear-gradient(orange, darkorange);">{ "Subscribe" }</button>
                    </form>
                </div>
            </div>
        </div>
    }
}

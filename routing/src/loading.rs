use yew::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="loading">
            <div class="loading__orange-container">
                <embed
                    src="/assets/images/Orange_Animated.svg"
                    type="image/svg+xml"
                    class="loading__orange-image"
                />
            </div>
            <div class="loading__progress"></div>
            <div class="loading__message">{"Loading, please wait..."}</div>
        </div>
    }
}

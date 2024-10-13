use yew::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div id="loading-container">
            <div id="orange-container">
                <embed
                    src="/assets/images/Orange_Animated.svg"
                    type="image/svg+xml"
                />
            </div>
            <div id="loading-progress"></div>
            <div id="loading-message">{"Loading, please wait..."}</div>

            <style>
                {r#"
                @keyframes fadeOut {
                    from { opacity: 1; }
                    to { opacity: 0; }
                }

                #loading-container {
                    display: flex;
                    position: fixed;
                    top: -1.15rem;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: rgb(255, 255, 255);
                    z-index: 1000;
                    justify-content: center;
                    align-items: center;
                    flex-direction: column;
                }

                #loading-message {
                    color: #121212;
                    margin-bottom: 20px;
                    font-size: 1.2rem;
                }

                #loading-progress {
                    background: linear-gradient(to right, #ff7e00, #ffbb00);
                    padding: 1rem 2rem;
                    width: 50%;
                    height: 30px;
                    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                }

                #orange-container {
                    margin: 5rem;
                    margin-bottom: 0;
                }

                #loading-container.loaded {
                    animation: fadeOut 1s;
                    animation-fill-mode: forwards;
                    pointer-events: none;
                }
                "#}
            </style>
        </div>
    }
}

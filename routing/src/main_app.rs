use gloo::utils::window;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::js_sys;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
enum AppJsState {
    NotLoaded,
    Loading,
    Loaded,
    Error(String),
}

#[function_component(MainApp)]
pub fn main_app() -> Html {
    let app_js_state = use_state(|| AppJsState::NotLoaded);

    {
        let app_js_state = app_js_state.clone();

        use_effect_with((), move |_| {
            if *app_js_state == AppJsState::NotLoaded {
                app_js_state.set(AppJsState::Loading);

                wasm_bindgen_futures::spawn_local(async move {
                    let window = window();

                    // Wait for render_app to be available
                    let mut attempts = 0;
                    const MAX_ATTEMPTS: u32 = 20;

                    while attempts < MAX_ATTEMPTS {
                        match js_sys::Reflect::get(&window, &JsValue::from_str("render_app")) {
                            Ok(render_app_fn) => {
                                if !render_app_fn.is_undefined() && !render_app_fn.is_null() {
                                    if let Ok(render_app) =
                                        render_app_fn.dyn_into::<js_sys::Function>()
                                    {
                                        let _ = render_app
                                            .call1(&JsValue::NULL, &JsValue::from_str("#app"));
                                        app_js_state.set(AppJsState::Loaded);
                                        return;
                                    }
                                }
                            }
                            Err(_) => {}
                        }

                        attempts += 1;
                        gloo::timers::future::TimeoutFuture::new(150).await;
                    }

                    // If we've reached this point, we couldn't load the app
                    app_js_state.set(AppJsState::Error(
                        "Failed to load render_app function after multiple attempts".into(),
                    ));
                });
            }
            || ()
        });
    }

    match (*app_js_state).clone() {
        AppJsState::Loaded => html! { <div id="app"></div> },
        AppJsState::Error(err) => html! { <div id="app">{"Error loading main app: "}{err}</div> },
        AppJsState::Loading => html! { <div id="app">{"Loading..."}</div> },
        AppJsState::NotLoaded => html! { <div id="app">{"Preparing to load main app..."}</div> },
    }
}

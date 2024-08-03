use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EmojisProps {
    pub img_src: String,
}

#[function_component(Emojis)]
pub fn emojis(props: &EmojisProps) -> Html {
    let svg_container_ref = use_node_ref();

    {
        let img_src = props.img_src.clone();
        let svg_container_ref = svg_container_ref.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let svg_content = fetch_svg_content(&img_src).await;
                if let Some(svg_content) = svg_content {
                    if let Some(svg_container) = svg_container_ref.cast::<Element>() {
                        svg_container.set_inner_html(&svg_content);
                    }
                }
            });
            || ()
        })
    }

    html! {
        <div class="emojis" ref={svg_container_ref}></div>
    }
}

async fn fetch_svg_content(url: &str) -> Option<String> {
    let response = gloo::net::http::Request::get(url).send().await.ok()?;
    response.text().await.ok()
}

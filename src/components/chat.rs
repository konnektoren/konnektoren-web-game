use crate::config::CHAT_API_URL;
use konnektoren_yew::prelude::use_profile;
use std::sync::Arc;
use yew::prelude::*;
use yew_chat::prelude::{ChatApp, MessageHandler, RequestMessageHandler};

#[derive(Properties, Clone, PartialEq)]
pub struct ChatProps {
    pub channel: String,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
    let profile = use_profile();
    let channel = props.channel.clone();
    let handler = Arc::new(RequestMessageHandler {
        host: CHAT_API_URL.to_string(),
    }) as Arc<dyn MessageHandler>;

    let expanded = use_state(|| false);

    let on_toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    html! {
        <div class={classes!("chat-popup", if *expanded { "expanded" } else { "" })}>
            <div class="chat-bubble" onclick={on_toggle.clone()}>
                <span class="chat-icon">{"💬"}</span>
                <span class="chat-text">{"Chat"}</span>
            </div>
            if *expanded {
                <div class="chat-content">
                    <button class="close-button" onclick={on_toggle}>{"×"}</button>
                    <ChatApp user={profile.name.clone()} channel={channel.clone()} handler={handler.clone()} />
                </div>
            }
        </div>
    }
}

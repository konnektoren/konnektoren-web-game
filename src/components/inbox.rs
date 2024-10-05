use crate::model::Inbox;
use chrono::Utc;
use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

const INBOX_STORAGE_KEY: &str = "konnektoren_inbox";

#[function_component(InboxComponent)]
pub fn inbox_component() -> Html {
    let inbox_state = use_state(|| Inbox {
        messages: vec![],
        read_messages: Some(vec![]),
    });
    let is_open = use_state(|| false);
    let unread_count = use_state(|| 0);

    {
        let inbox_state = inbox_state.clone();
        let unread_count = unread_count.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let yaml_content = Request::get("/assets/inbox.yml")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let mut fetched_inbox: Inbox = serde_yaml::from_str(&yaml_content).unwrap();

                // Load messages from local storage
                if let Ok(stored_inbox) = LocalStorage::get::<Inbox>(INBOX_STORAGE_KEY) {
                    fetched_inbox.merge(&stored_inbox);
                }

                fetched_inbox
                    .messages
                    .sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

                let unread = fetched_inbox.messages.len()
                    - fetched_inbox.read_messages.as_ref().map_or(0, |v| v.len());

                inbox_state.set(fetched_inbox.clone());
                unread_count.set(unread);

                // Save the merged inbox back to local storage
                LocalStorage::set(INBOX_STORAGE_KEY, fetched_inbox).unwrap();
            });
            || ()
        });
    }

    let toggle_inbox = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let mark_as_read = {
        let inbox_state = inbox_state.clone();
        let unread_count = unread_count.clone();
        Callback::from(move |message_id: String| {
            if let Ok(stored_inbox) = LocalStorage::get::<Inbox>(INBOX_STORAGE_KEY) {
                let mut current_inbox = stored_inbox;
                let read_messages = current_inbox.read_messages.get_or_insert_with(Vec::new);
                if !read_messages.contains(&message_id) {
                    read_messages.push(message_id);
                    inbox_state.set(current_inbox.clone());
                    LocalStorage::set(INBOX_STORAGE_KEY, current_inbox).unwrap();
                    let current_unread = *unread_count;
                    unread_count.set(current_unread - 1);
                }
            }
        })
    };

    html! {
        <div class={classes!("inbox-component", if *is_open { "open" } else { "" })}>
            if *is_open {
                <div class="inbox-content">
                    <button class="close-button" onclick={toggle_inbox.clone()}>{"×"}</button>
                    <h2>{"Inbox"}</h2>
                    <div class="message-list">
                        {for inbox_state.messages.iter().map(|message| {
                            let is_read = inbox_state.read_messages.as_ref()
                                .map(|read| read.contains(&message.id.clone().unwrap_or_default()))
                                .unwrap_or(false);
                            let mark_as_read = mark_as_read.clone();
                            let message_id = message.id.clone().unwrap_or_default();
                            html! {
                                <div
                                    class={classes!("message", if is_read { "read" } else { "unread" })}
                                    onclick={Callback::from(move |_| mark_as_read.emit(message_id.clone()))}
                                >
                                    <div class="message-header">
                                        <span class="sender">{&message.sender}</span>
                                        <span class="timestamp">{message.timestamp.with_timezone(&Utc).format("%Y-%m-%d %H:%M").to_string()}</span>
                                    </div>
                                    <div class="message-content">{&message.content}</div>
                                </div>
                            }
                        })}
                    </div>
                </div>
            }
            <div class="inbox-icon" onclick={toggle_inbox}>
                <i class="fa-solid fa-envelope"></i>
                if *unread_count > 0 {
                    <span class="unread-count">{*unread_count}</span>
                }
            </div>
        </div>
    }
}

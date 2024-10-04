use serde::{Deserialize, Serialize};
use yew_chat::prelude::Message;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inbox {
    pub messages: Vec<Message>,
    pub read_messages: Option<Vec<String>>,
}

use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MsgIn {

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum MsgOut {
    Connected {
        version: String,
    }
}

impl Into<Message> for MsgOut {
    fn into(self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
    }
}



use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use crate::api::models::{network::Network, server::Server, Source};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum MsgIn {
    AddSource {
        to: ServerOrNetwork,
        source: Source,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ServerOrNetwork {
    Server,
    Network,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum MsgOut {
    Connected {
        version: String,
    },

    AppInfo {
        server: Option<Server>,
        network: Option<Network>,
    },
}

impl Into<Message> for MsgOut {
    fn into(self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
    }
}

use serde::{Deserialize, Serialize};

pub const NETWORK_TOML: &str = "network.toml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    pub name: String,
}

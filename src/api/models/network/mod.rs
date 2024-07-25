use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Source;

pub const NETWORK_TOML: &str = "network.toml";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Network {
    pub name: String,
    pub variables: HashMap<String, String>,

    pub servers: HashMap<String, ServerEntry>,
    pub groups: HashMap<String, NetworkGroup>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ServerEntry {
    pub groups: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct NetworkGroup {
    #[serde(default = "Vec::<Source>::new")]
    pub sources: Vec<Source>,
}

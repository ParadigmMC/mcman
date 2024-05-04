use serde::{Serialize, Deserialize};

use super::AddonSource;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct Server {
    name: String,
    port: Option<i32>,

    sources: Vec<AddonSource>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            name: String::from("server"),
            port: None,

            sources: vec![],
        }
    }
}

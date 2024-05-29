use serde::{Serialize, Deserialize};

use super::AddonSource;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct Server {
    pub name: String,
    pub port: Option<i32>,

    pub sources: Vec<AddonSource>,
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

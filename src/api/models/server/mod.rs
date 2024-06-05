use serde::{Serialize, Deserialize};

use super::Source;

mod server_type;
mod server_flavor;

pub use server_type::*;
pub use server_flavor::*;

pub const SERVER_TOML: &str = "server.toml";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct Server {
    pub name: String,
    pub port: Option<i32>,

    #[serde(default = "Vec::new")]
    pub sources: Vec<Source>,
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

use serde::{Deserialize, Serialize};

use super::Source;

mod server_flavor;
mod server_type;

pub use server_flavor::*;
pub use server_type::*;

pub const SERVER_TOML: &str = "server.toml";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct Server {
    pub name: String,
    pub port: Option<i32>,

    pub jar: Option<ServerJar>,

    #[serde(default = "Vec::new")]
    pub sources: Vec<Source>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            name: String::from("server"),
            port: None,

            jar: Some(ServerJar {
                mc_version: String::from("1.20.4"),
                server_type: ServerType::Vanilla {},
            }),

            sources: vec![],
        }
    }
}

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{launcher::ServerLauncher, markdown::MarkdownOptions, Source};

mod server_flavor;
mod server_type;

pub use server_flavor::*;
pub use server_type::*;

pub const SERVER_TOML: &str = "server.toml";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, JsonSchema)]
#[serde(default)]
pub struct Server {
    pub name: String,
    pub port: Option<i32>,

    pub jar: Option<ServerJar>,

    #[serde(default = "Vec::<Source>::new")]
    pub sources: Vec<Source>,

    #[serde(default = "HashMap::<String, String>::new")]
    pub variables: HashMap<String, String>,

    #[serde(default)]
    pub markdown: MarkdownOptions,    
    
    #[serde(default)]
    pub launcher: ServerLauncher,
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

            markdown: MarkdownOptions::default(),
            launcher: ServerLauncher::default(),

            sources: vec![],
            variables: HashMap::default(),
        }
    }
}

impl Server {
    pub fn get_execution_arguments(&self) -> Vec<String> {
        self.jar.as_ref().map(|s| s.get_execution_arguments()).unwrap_or_default()
    }

    pub fn get_arguments(&self) -> Vec<String> {
        let mut args = vec![];

        args.extend(self.launcher.jvm_args.split_whitespace().map(ToOwned::to_owned));

        // TODO: -Xmx -Xms

        args.extend(self.launcher.preset_flags.get_flags());

        if self.launcher.eula_args && self.jar.as_ref().is_some_and(|x| x.flavor().supports_eula_args()) {
            args.push(String::from("-Dcom.mojang.eula.agree=true"));
        }

        for (key, value) in &self.launcher.properties {
            let value = serde_json::to_string(value).unwrap();

            args.push(format!("-D{key}={value}"));
        }

        args.extend(self.get_execution_arguments());

        if self.launcher.nogui && self.jar.as_ref().is_some_and(|x| x.flavor().supports_nogui()) {
            args.push(String::from("--nogui"));
        }

        args.extend(self.launcher.game_args.split_whitespace().map(ToOwned::to_owned));

        args
    }
}

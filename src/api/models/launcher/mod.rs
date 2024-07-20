use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{borrow::ToOwned, collections::HashMap, env};

use crate::api::utils::serde::*;

mod preset_flags;
pub use preset_flags::*;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, JsonSchema)]
#[serde(default)]
pub struct ServerLauncher {
    pub eula_args: bool,

    #[serde(skip_serializing_if = "is_default")]
    pub nogui: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub preset_flags: PresetFlags,
    #[serde(skip_serializing_if = "is_default")]
    pub disable: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub jvm_args: String,
    #[serde(skip_serializing_if = "is_default")]
    pub game_args: String,
    #[serde(skip_serializing_if = "is_default")]
    pub memory: String,
    #[serde(skip_serializing_if = "is_default")]
    pub properties: HashMap<String, String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prelaunch: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub postlaunch: Vec<String>,

    pub java_version: Option<u32>,
}

impl ServerLauncher {
    pub fn get_args(&self, exec: &str) -> Vec<String> {
        let mut args = self
            .jvm_args
            .split_whitespace()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();

        if env::var("MC_MEMORY").is_ok() || !self.memory.is_empty() {
            let m = env::var("MC_MEMORY").unwrap_or(self.memory.clone());
            args.extend([format!("-Xms{m}"), format!("-Xmx{m}")]);
        }

        args.append(&mut self.preset_flags.get_flags());

        if self.eula_args {
            args.push(String::from("-Dcom.mojang.eula.agree=true"));
        }

        for (key, value) in &self.properties {
            args.push(format!(
                "-D{}={}",
                key,
                if value.contains(char::is_whitespace) {
                    format!("\"{value}\"")
                } else {
                    value.clone()
                }
            ));
        }

        args.extend(exec.split_whitespace().map(ToOwned::to_owned));

        if self.nogui && !matches!(self.preset_flags, PresetFlags::Proxy) {
            args.push(String::from("--nogui"));
        }
        
        args.extend(self.game_args.split_whitespace().map(ToOwned::to_owned));

        args
    }
}

impl Default for ServerLauncher {
    fn default() -> Self {
        Self {
            preset_flags: PresetFlags::None,
            nogui: true,
            jvm_args: String::new(),
            game_args: String::new(),
            disable: false,
            eula_args: true,
            memory: String::new(),
            properties: HashMap::default(),
            prelaunch: vec![],
            postlaunch: vec![],
            java_version: None,
        }
    }
}

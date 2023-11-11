use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct ServerLauncher {
    pub aikars_flags: bool,
    pub proxy_flags: bool,
    pub nogui: bool,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub disable: bool,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub jvm_args: String,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub game_args: String,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub eula_args: bool,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub memory: String,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum StartupMethod {
    Jar(String),
    Custom {
        windows: Vec<String>,
        linux: Vec<String>,
    },
}

impl ServerLauncher {
    pub fn generate_script_linux(&self, _servername: &str, startup: &StartupMethod) -> String {
        format!(
            "#!/bin/sh\n# generated by mcman\njava {} \"$@\"\n",
            self.get_arguments(startup, "linux").join(" ")
        )
    }

    pub fn generate_script_win(&self, servername: &str, startup: &StartupMethod) -> String {
        format!(
            "@echo off\r\n:: generated by mcman\r\ntitle {servername}\r\njava {} %*\r\n",
            self.get_arguments(startup, "windows").join(" ")
        )
    }

    pub fn get_arguments(&self, startup: &StartupMethod, platform: &str) -> Vec<String> {
        let mut args = vec![];

        for arg in self.jvm_args.split_whitespace() {
            args.push(arg.to_owned());
        }

        if std::env::var("MC_MEMORY").is_ok() || !self.memory.is_empty() {
            let m = std::env::var("MC_MEMORY").unwrap_or(self.memory.clone());
            args.push(format!("-Xms{m}"));
            args.push(format!("-Xmx{m}"));
        }

        if self.aikars_flags {
            args.append(
                &mut include_str!("../../res/aikars_flags")
                    .split(char::is_whitespace)
                    .map(ToOwned::to_owned)
                    .collect(),
            );
        }

        if self.proxy_flags {
            args.append(
                &mut include_str!("../../res/proxy_flags")
                    .split(char::is_whitespace)
                    .map(ToOwned::to_owned)
                    .collect(),
            );
        }

        if self.eula_args {
            args.push(String::from("-Dcom.mojang.eula.agree=true"));
        }

        for (key, value) in &self.properties {
            args.push(format!(
                "-D{}={}",
                key,
                if value.contains(char::is_whitespace) {
                    "\"".to_owned() + value + "\""
                } else {
                    value.clone()
                }
            ));
        }

        match startup.clone() {
            StartupMethod::Jar(jar) => {
                args.push(String::from("-jar"));
                args.push(jar);
            }
            StartupMethod::Custom { linux, windows } => {
                for item in match platform {
                    "linux" => linux,
                    "windows" => windows,
                    _ => vec![],
                } {
                    args.push(item);
                }
            }
        }

        if self.nogui {
            args.push(String::from("--nogui"));
        }

        for arg in self.game_args.split_whitespace() {
            args.push(arg.to_owned());
        }

        args
    }
}

impl Default for ServerLauncher {
    fn default() -> Self {
        Self {
            disable: false,
            jvm_args: String::new(),
            game_args: String::new(),
            aikars_flags: true,
            proxy_flags: false,
            nogui: true,
            eula_args: true,
            memory: String::new(),
            properties: HashMap::default(),
        }
    }
}

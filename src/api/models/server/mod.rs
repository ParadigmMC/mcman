use std::collections::HashMap;

use anyhow::{anyhow, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::api::{app::App, tools::java::get_java_installation_for};

use super::{launcher::ServerLauncher, markdown::MarkdownOptions, mrpack::resolve_mrpack_serverjar, packwiz::resolve_packwiz_serverjar, ModpackType, Source, SourceType};

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
    /// Gets the ServerJar via `jar` OR `Source` where `type=modpack`
    pub async fn get_jar(&self, app: &App) -> Result<ServerJar> {
        let relative_to = app.server.read().await.as_ref().map(|(p, _)| p.clone());

        if let Some(jar) = &self.jar {
            Ok(jar.clone())
        } else {
            let source = self.sources.iter().find(|s| matches!(s.source_type, SourceType::Modpack { .. }))
            .ok_or(anyhow!("Can't find a ServerJar type because no [jar] OR Source with type=modpack defined"))?;

            let accessor = source.accessor(&relative_to.ok_or(anyhow!("relative_to error"))?)?;
            match source.modpack_type().unwrap() {
                ModpackType::MRPack => resolve_mrpack_serverjar(app, accessor).await,
                ModpackType::Packwiz => resolve_packwiz_serverjar(app, accessor).await,
                ModpackType::Unsup => todo!()
            }
        }
    }

    pub async fn get_java(&self) -> String {
        if let Some(v) = self.launcher.java_version {
            get_java_installation_for(v).await.map(|j| j.path.to_string_lossy().into_owned()).unwrap_or(String::from("java"))
        } else {
            String::from("java")
        }
    }

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

        args.into_iter().filter(|x| !x.is_empty()).collect()
    }
}

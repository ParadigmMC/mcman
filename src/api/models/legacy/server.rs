use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::models::{hooks::Hook, launcher::ServerLauncher};

use super::{
    LegacyClientSideMod, LegacyDownloadable, LegacyMarkdownOptions, LegacyServerOptions,
    LegacyServerType, LegacyWorld,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct LegacyServer {
    pub name: String,
    pub mc_version: String,
    #[serde(with = "super::server_type")]
    pub jar: LegacyServerType,
    pub variables: HashMap<String, String>,
    pub launcher: ServerLauncher,

    #[serde(default)]
    #[serde(skip_serializing_if = "LegacyMarkdownOptions::is_empty")]
    pub markdown: LegacyMarkdownOptions,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub hooks: HashMap<String, Hook>,

    #[serde(default)]
    pub options: LegacyServerOptions,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub worlds: HashMap<String, LegacyWorld>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<LegacyDownloadable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mods: Vec<LegacyDownloadable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub clientsidemods: Vec<LegacyClientSideMod>,
}

impl Default for LegacyServer {
    fn default() -> Self {
        let mut vars = HashMap::new();
        vars.insert("SERVER_PORT".to_owned(), "25565".to_owned());
        Self {
            name: String::new(),
            mc_version: "latest".to_owned(),
            jar: LegacyServerType::Vanilla {},
            variables: vars,
            launcher: ServerLauncher::default(),
            markdown: LegacyMarkdownOptions::default(),
            hooks: HashMap::new(),
            options: LegacyServerOptions::default(),
            worlds: HashMap::new(),
            plugins: vec![],
            mods: vec![],
            clientsidemods: vec![],
        }
    }
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::models::hooks::Hook;

use super::{LegacyDownloadable, LegacyMarkdownOptions};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct LegacyNetwork {
    pub name: String,
    pub proxy: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub proxy_groups: Vec<String>,
    pub port: u16,
    pub servers: HashMap<String, LegacyServerEntry>,
    pub variables: HashMap<String, String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "LegacyMarkdownOptions::is_empty")]
    pub markdown: LegacyMarkdownOptions,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub hooks: HashMap<String, Hook>,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub groups: HashMap<String, LegacyGroup>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct LegacyServerEntry {
    pub port: u16,
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct LegacyGroup {
    #[serde(default)]
    pub variables: HashMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<LegacyDownloadable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mods: Vec<LegacyDownloadable>,
}

impl Default for LegacyNetwork {
    fn default() -> Self {
        Self {
            name: String::new(),
            proxy: "proxy".to_owned(),
            proxy_groups: vec![],
            port: 25565,
            servers: HashMap::new(),
            variables: HashMap::new(),
            markdown: LegacyMarkdownOptions::default(),
            hooks: HashMap::new(),
            groups: HashMap::new(),
        }
    }
}

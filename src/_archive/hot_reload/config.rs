use std::{collections::HashMap, fs, str::FromStr, path::PathBuf};

use anyhow::{anyhow, Result};
use glob::Pattern;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(tag = "type", try_from = "String", into = "String")]
pub enum HotReloadAction {
    #[default]
    Reload,
    Restart,
    #[serde(alias = "run")]
    RunCommand(String),
}

impl FromStr for HotReloadAction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if value.starts_with('/') {
            Ok(Self::RunCommand(
                value.strip_prefix('/').unwrap().to_string(),
            ))
        } else {
            match value.to_lowercase().as_str() {
                "reload" => Ok(Self::Reload),
                "restart" => Ok(Self::Restart),
                _ => Err(anyhow!("Cant parse HotReloadAction: {value}")),
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotReloadConfig {
    #[serde(skip)]
    pub path: PathBuf,

    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::default")]
    pub files: Vec<Entry>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub events: HashMap<String, HotReloadAction>,
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./hotreload.toml"),
            events: HashMap::new(),
            files: vec![Entry {
                path: Pattern::new("server.properties").unwrap(),
                action: HotReloadAction::Reload,
            }],
        }
    }
}

impl HotReloadConfig {
    pub fn load_from(path: &PathBuf) -> Result<Self> {
        let data = fs::read_to_string(path)?;
        let mut h: Self = toml::from_str(&data)?;
        h.path = path.to_owned();
        Ok(h)
    }

    pub fn save(&self) -> Result<()> {
        Ok(fs::write(&self.path, toml::to_string_pretty(&self)?)?)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Entry {
    #[serde(with = "super::pattern_serde")]
    pub path: Pattern,
    pub action: HotReloadAction,
}

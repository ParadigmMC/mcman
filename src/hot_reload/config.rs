use std::{collections::HashMap, path::PathBuf, fs::File, io::Write};

use anyhow::Result;
use glob::Pattern;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HotReloadAction {
    Reload,
    Restart,
    #[serde(alias = "run")]
    RunCommand(String),
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
            files: vec![
                Entry {
                    path: Pattern::new("server.properties").unwrap(),
                    action: HotReloadAction::Reload,
                }
            ],
        }
    }
}

impl HotReloadConfig {
    pub fn load_from(path: &PathBuf) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let mut h: Self = toml::from_str(&data)?;
        h.path = path.to_owned();
        Ok(h)
    }

    pub fn reload(&mut self) -> Result<()> {
        let path = self.path.clone();
        let data = std::fs::read_to_string(path)?;
        let mut h: Self = toml::from_str(&data)?;
        self.events = h.events;
        self.files = h.files;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(&self.path)?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    #[serde(with = "super::pattern_serde")]
    pub path: Pattern,
    pub action: HotReloadAction,
}

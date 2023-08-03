use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HotReloadAction {
    Reload,
    Restart,
    ReloadPlugin(String),
    RunCommand(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotReloadConfig {
    pub files: Vec<Entry>,
    pub events: HashMap<HotReloadEvent, HotReloadAction>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HotReloadEvent {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub path: String,
    pub action: HotReloadAction,
}

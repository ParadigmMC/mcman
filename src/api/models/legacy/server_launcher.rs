use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::utils::serde::is_default;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum LegacyPresetFlags {
    Aikars,
    Proxy,
    #[default]
    None,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct LegacyServerLauncher {
    pub eula_args: bool,

    #[serde(skip_serializing_if = "is_default")]
    pub nogui: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub preset_flags: LegacyPresetFlags,
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

    pub java_version: Option<String>,
}

impl Default for LegacyServerLauncher {
    fn default() -> Self {
        Self {
            preset_flags: LegacyPresetFlags::None,
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

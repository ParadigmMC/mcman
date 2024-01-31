use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Hook {
    pub when: HookEvent,
    #[serde(default)]
    pub onfail: HookFailBehavior,
    #[serde(default = "bool_true")]
    pub show_output: bool,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub env: HashMap<String, String>,

    pub windows: Option<String>,
    pub linux: Option<String>,
}

fn bool_true() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum HookEvent {
    #[default]
    None,
    PreBuild,
    PostBuild,
    PreInstall,
    PostInstall,
    PreWorldUnpack,
    PostWorldUnpack,
    PreWorldPack,
    PostWorldPack,
    DevSessionStart,
    DevSessionEnd,
    TestSuccess,
    TestFail,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Default)]
pub enum HookFailBehavior {
    #[default]
    Error,
    Ignore,
    Warn,
}

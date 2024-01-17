use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Hook {
    pub when: HookEvent,
    pub onfail: HookFailBehavior,
    pub show_output: bool,
    pub description: String,
    pub disabled: bool,
    pub env: HashMap<String, String>,

    pub windows: Option<String>,
    pub linux: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Default)]
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

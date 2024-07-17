use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct LegacyServerOptions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bootstrap_exts: Vec<String>,

    #[serde(
        default = "default_success_line",
        skip_serializing_if = "is_default_success_line"
    )]
    pub success_line: String,

    #[serde(
        default = "default_stop_command",
        skip_serializing_if = "is_default_stop_command"
    )]
    pub stop_command: String,
}

pub fn default_success_line() -> String {
    String::from("]: Done")
}

pub fn is_default_success_line(s: &str) -> bool {
    s == default_success_line()
}

pub fn default_stop_command() -> String {
    String::from("stop")
}

pub fn is_default_stop_command(s: &str) -> bool {
    s == default_stop_command()
}

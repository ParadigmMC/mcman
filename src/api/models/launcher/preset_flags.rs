use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PresetFlags {
    Aikars,
    Proxy,
    #[default]
    None,
}

impl PresetFlags {
    pub fn get_flags(&self) -> Vec<String> {
        match self {
            Self::Aikars => include_str!("../../../../res/aikars_flags"),
            Self::Proxy => include_str!("../../../../res/proxy_flags"),
            Self::None => "",
        }
        .split(char::is_whitespace)
        .map(ToOwned::to_owned)
        .collect()
    }
}
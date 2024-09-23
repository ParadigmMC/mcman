use crate::api::utils::serde::str_latest;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::LegacyDownloadable;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LegacyServerType {
    Vanilla {},

    PaperMC {
        project: String,
        #[serde(default = "str_latest")]
        build: String,
    },

    Purpur {
        #[serde(default = "str_latest")]
        build: String,
    },

    Fabric {
        #[serde(default = "str_latest")]
        loader: String,

        #[serde(default = "str_latest")]
        installer: String,
    },

    Quilt {
        #[serde(default = "str_latest")]
        loader: String,

        #[serde(default = "str_latest")]
        installer: String,
    },

    NeoForge {
        #[serde(default = "str_latest")]
        loader: String,
    },

    Forge {
        #[serde(default = "str_latest")]
        loader: String,
    },

    BuildTools {
        #[serde(default = "str_spigot")]
        software: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[serde(default = "Vec::new")]
        args: Vec<String>,
    },

    Paper {},
    Velocity {},
    Waterfall {},
    BungeeCord {},

    Downloadable {
        #[serde(flatten)]
        inner: LegacyDownloadable,
    },
}

pub fn str_spigot() -> String {
    String::from("spigot")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Bridge {
    ServerType(LegacyServerType),
    Downloadable(LegacyDownloadable),
}

impl From<Bridge> for LegacyServerType {
    fn from(value: Bridge) -> Self {
        match value {
            Bridge::ServerType(ty) => ty,
            Bridge::Downloadable(d) => Self::Downloadable { inner: d },
        }
    }
}

pub fn serialize<S>(st: &LegacyServerType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    st.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<LegacyServerType, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(LegacyServerType::from(Bridge::deserialize(deserializer)?))
}

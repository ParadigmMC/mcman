use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LegacyDownloadable {
    // sources
    Url {
        url: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "crate::api::utils::serde::is_default")]
        filename: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "crate::api::utils::serde::is_default")]
        desc: Option<String>,
    },

    #[serde(alias = "mr")]
    Modrinth {
        id: String,
        #[serde(default = "latest")]
        version: String,
    },

    #[serde(alias = "cr")]
    CurseRinth {
        id: String,
        #[serde(default = "latest")]
        version: String,
    },

    Spigot {
        id: String,
        #[serde(default = "latest")]
        version: String,
    },

    Hangar {
        id: String,
        version: String,
    },

    #[serde(rename = "ghrel")]
    GithubRelease {
        repo: String,
        tag: String,
        asset: String,
    },

    // pain in the a-
    Jenkins {
        url: String,
        job: String,
        #[serde(default = "latest")]
        build: String,
        #[serde(default = "first")]
        artifact: String,
    },

    Maven {
        url: String,
        group: String,
        artifact: String,
        #[serde(default = "latest")]
        version: String,
        #[serde(default = "artifact")]
        filename: String,
    },
}

pub fn latest() -> String {
    "latest".to_owned()
}

pub fn first() -> String {
    "first".to_owned()
}

pub fn artifact() -> String {
    "artifact".to_owned()
}

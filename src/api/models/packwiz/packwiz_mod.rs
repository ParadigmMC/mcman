use serde::{Deserialize, Serialize};

use crate::api::{models::Environment, utils::hashing::HashFormat};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
#[serde(default)]
pub struct PackwizMod {
    pub name: String,
    pub filename: String,
    pub download: PackwizModDownload,
    pub option: PackwizModOption,
    pub side: Environment,
    pub update: Option<PackwizModUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct PackwizModOption {
    pub optional: bool,
    pub default: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "kebab-case")]
pub struct PackwizModDownload {
    pub url: Option<String>,
    pub hash: String,
    pub hash_format: HashFormat,
    #[serde(default)]
    pub mode: PackwizDownloadMode,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum PackwizDownloadMode {
    #[default]
    #[serde(alias = "")]
    Url,
    #[serde(rename = "metadata:curseforge")]
    Curseforge,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PackwizModUpdate {
    #[serde(rename_all = "kebab-case")]
    Modrinth { mod_id: String, version: String },
    #[serde(rename_all = "kebab-case")]
    Curseforge { file_id: u64, project_id: u64 },
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::utils::hashing::HashFormat;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct PackwizPack {
    pub name: String,
    pub author: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub pack_format: String,
    pub index: PackwizPackFile,
    pub versions: HashMap<String, String>,
}

pub static PACK_TOML: &str = "pack.toml";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct PackwizPackIndex {
    pub hash_format: HashFormat,
    pub files: Vec<PackwizPackFile>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct PackwizPackFile {
    #[serde(rename = "file")]
    pub path: String,
    pub hash: String,
    pub hash_format: Option<String>,

    pub alias: Option<String>,
    #[serde(default)]
    pub metafile: bool,
    #[serde(default)]
    pub preserve: bool,
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::{models::Environment, utils::hashing::HashFormat};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MRPackFile {
    pub path: String,
    pub hashes: HashMap<HashFormat, String>,
    pub env: Option<Env>,
    pub file_size: u64,
    pub downloads: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Env {
    pub client: EnvSupport,
    pub server: EnvSupport,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EnvSupport {
    Required,
    Optional,
    Unsupported,
}

use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::App;

#[async_trait]
pub trait Resolvable {
    async fn resolve_source(&self, app: &App) -> Result<ResolvedFile>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedFile {
    pub url: String,
    pub filename: String,
    pub cache: CacheStrategy,
    pub size: Option<u64>,
    pub hashes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "type")]
pub enum CacheStrategy {
    File {
        namespace: String,
        path: String,
    },
    Indexed {
        index_path: String,
        key: String,
        value: String,
    },
    #[default]
    None,
}

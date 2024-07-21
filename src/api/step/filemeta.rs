use std::collections::HashMap;

use digest::DynDigest;
use serde::{Deserialize, Serialize};

use crate::api::utils::hashing::{get_best_hash, HashFormat};

use super::CacheLocation;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileMeta {
    pub filename: String,
    pub size: Option<u64>,
    pub hashes: HashMap<HashFormat, String>,
    pub cache: Option<CacheLocation>,
}

impl FileMeta {
    pub fn filename(filename: String) -> Self {
        Self {
            filename,
            ..Default::default()
        }
    }

    pub fn get_hasher(&self) -> Option<(HashFormat, Box<dyn DynDigest + Send>, String)> {
        get_best_hash(&self.hashes).map(|(format, content)| (format, format.get_digest(), content))
    }
}

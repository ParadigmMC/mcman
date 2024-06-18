use std::{borrow::Cow, collections::HashMap, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::utils::hashing::HashFormat;

use super::utils::hashing::get_best_hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Step {
    CacheCheck(FileMeta),
    Download { url: String, metadata: FileMeta },
    Execute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheLocation(pub Cow<'static, str>, pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileMeta {
    pub filename: String,
    pub size: Option<u64>,
    pub hashes: HashMap<HashFormat, String>,
    pub cache: Option<CacheLocation>,
}

impl FileMeta {
    pub fn does_match<P: AsRef<Path>>(&self, file: P) -> Result<bool> {
        let metadata = std::fs::metadata(file.as_ref())?;

        if self.size.is_some_and(|size| metadata.len() != size) {
            return Ok(false);
        }

        if let Some((format, hash)) = get_best_hash(&self.hashes) {}

        Ok(true)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepResult {
    // continue into running next step
    #[default]
    Continue,
    // skip next steps for this addon
    // example: addon is already downloaded / cache hit
    Skip,
}

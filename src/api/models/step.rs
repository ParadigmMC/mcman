use std::{borrow::Cow, collections::HashMap};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::utils::hashing::HashFormat;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Step {
    CacheCheck(CacheStrategy),
    Download {
        url: String,
        filename: String,
        size: Option<u64>,
        hashes: HashMap<HashFormat, String>,
    },
    Execute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CacheStrategy {
    File {
        namespace: Cow<'static, str>,
        path: String,
    },
    Indexed {
        namespace: Cow<'static, str>,
        path: Option<String>,
        key: String,
    },
}

pub enum StepResult {
    // continue into running next step
    Continue,
    // skip next steps for this addon
    // example: addon is already downloaded / cache hit
    Skip,
}

impl Step {
    async fn run(&self) -> Result<StepResult> {
        Ok(StepResult::Continue)
    }
}

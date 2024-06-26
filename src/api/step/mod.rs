use serde::{Deserialize, Serialize};

mod filemeta;
mod cache_location;

pub use filemeta::*;
pub use cache_location::*;

use super::tools::java::JavaVersion;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Step {
    CacheCheck(FileMeta),
    Download { url: String, metadata: FileMeta },
    ExecuteJava {
        args: Vec<String>,
        java_version: Option<JavaVersion>,
        label: String,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepResult {
    // continue into running next step
    #[default]
    Continue,
    // skip next steps for this addon
    // example: cache hit
    Skip,
}

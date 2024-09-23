use serde::{Deserialize, Serialize};

mod cache_location;
mod filemeta;

pub use cache_location::*;
pub use filemeta::*;

use super::tools::java::JavaVersion;

/// A step is the building block of doing things
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Step {
    /// Check the cache directory for a file;
    /// Copy over the file if it's in cache and skip the next step (which is a `Step::Download` in most cases)
    CacheCheck(FileMeta),
    /// Download a file from an URL.
    /// If `metadata.cache` is Some, download to cache directory and copy the file from there
    Download { url: String, metadata: FileMeta },
    /// Execute a java program
    ExecuteJava {
        args: Vec<String>,
        java_version: Option<JavaVersion>,
        label: String,
    },
    /// Remove/delete a file
    RemoveFile(FileMeta),
}

/// Result of executing a step
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepResult {
    /// Continue executing to next step
    #[default]
    Continue,
    /// Skip the next step
    Skip,
}

use std::path::Path;

use anyhow::{anyhow, Context, Result};

pub async fn create_parents(path: &Path) -> Result<()> {
    let parent = path.parent().ok_or(anyhow!("Getting parent of: {path:?}"))?;
    tokio::fs::create_dir_all(parent)
        .await
        .with_context(|| format!("Creating directory: {parent:?}"))
}

pub fn create_parents_sync(path: &Path) -> Result<()> {
    let parent = path.parent().ok_or(anyhow!("Getting parent of: {path:?}"))?;
    std::fs::create_dir_all(parent)
        .with_context(|| format!("Creating directory: {parent:?}"))
}

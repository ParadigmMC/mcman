use std::{ffi::OsStr, path::{Path, PathBuf}};

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

pub fn some_if_exists<T: ?Sized + AsRef<OsStr>>(path: &T) -> Option<PathBuf> {
    let v = PathBuf::from(path);
    if v.exists() {
        Some(v)
    } else {
        None
    }
}

pub fn with_extension_if_none<T: ?Sized + AsRef<OsStr>>(path: &T, ext: &str) -> PathBuf {
    let path = PathBuf::from(path);
    if path.extension().is_some_and(|e| e == ext) {
        path
    } else {
        path.with_extension(ext)
    }
}

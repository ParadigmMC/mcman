use std::{path::PathBuf, fs::{self, File}, io::Write};

use anyhow::{Result, Context};
use serde::de::DeserializeOwned;

pub struct Cache(pub PathBuf);

impl Cache {
    pub fn cache_root() -> Option<PathBuf> {
        Some(dirs::cache_dir()?.join("mcman"))
    }

    pub fn get_cache(namespace: &str) -> Option<Self> {
        let dir = Self::cache_root()?.join(namespace);
        Some(Self(dir))
    }

    pub fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let content = fs::read_to_string(self.0.join(path))?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn path(&self, path: &str) -> PathBuf {
        self.0.join(path)
    }

    pub fn exists(&self, path: &str) -> bool {
        self.path(path).exists()
    }

    pub fn try_get_json<T: DeserializeOwned>(&self, path: &str) -> Result<Option<T>> {
        if self.exists(path) {
            Ok(Some(self.get_json(path)?))
        } else {
            Ok(None)
        }
    }

    pub fn write_json<T: serde::Serialize>(&self, path: &str, data: &T) -> Result<()> {
        fs::create_dir_all(self.path(path).parent().unwrap())
            .context(format!("Creating parent directory for: {path}"))?;
        let content = serde_json::to_string(data)?;
        let mut f = File::create(self.path(path))
            .context(format!("Creating cache file at: {path}"))?;
        f.write_all(content.as_bytes())?;

        Ok(())
    }
}

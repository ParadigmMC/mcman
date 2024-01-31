use std::{
    fs::{self, File},
    io::{BufReader, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
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
        let file = File::open(self.0.join(path))?;
        let reader = BufReader::new(file);

        Ok(serde_json::from_reader(reader)?)
    }

    pub fn path(&self, path: &str) -> PathBuf {
        self.0.join(path)
    }

    pub fn exists(&self, path: &str) -> bool {
        self.path(path).exists()
    }

    pub fn try_get_json<T: DeserializeOwned>(&self, path: &str) -> Result<Option<T>> {
        Ok(if self.exists(path) {
            Some(self.get_json(path)?)
        } else {
            None
        })
    }

    pub fn write_json<T: serde::Serialize>(&self, path: &str, data: &T) -> Result<()> {
        let writer = BufWriter::new(
            File::create(self.path(path)).context(format!("Creating cache file at: {path}"))?,
        );

        Ok(serde_json::to_writer(writer, data)?)
    }
}

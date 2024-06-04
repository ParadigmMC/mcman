use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;

pub struct Cache(pub Option<PathBuf>);

impl Cache {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self(path)
    }

    pub fn join(&self, path: &str) -> Option<PathBuf> {
        Some(self.0.as_ref()?.join(path))
    }

    pub fn try_get_json<T: DeserializeOwned>(&self, path: &str) -> Result<Option<T>> {
        match &self.0 {
            Some(base) => {
                let fullpath = base.join(path);

                if !fullpath.exists() {
                    return Ok(None)
                }

                let file = File::open(fullpath)?;
                let reader = BufReader::new(file);
        
                Ok(serde_json::from_reader(reader)?)
            },

            None => Ok(None),
        }
    }

    pub fn try_write_json<T: serde::Serialize>(&self, path: &str, data: &T) -> Result<()> {
        match &self.0 {
            Some(base) => {
                let fullpath = base.join(path);

                let writer = BufWriter::new(
                    File::create(&fullpath)
                    .context(format!("Creating cache file at: {}", fullpath.display()))?,
                );

                serde_json::to_writer(writer, data)?;
        
                Ok(())
            }

            _ => Ok(())
        }
    }
}

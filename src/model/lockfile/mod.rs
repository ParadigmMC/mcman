use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf, time::SystemTime,
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use super::Server;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Lockfile {
    #[serde(skip)]
    pub path: PathBuf,

    pub server: Server,
    pub files: Vec<BootstrappedFile>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BootstrappedFile {
    pub path: PathBuf,
    pub date: SystemTime,
}

impl Lockfile {
    pub fn load_from(path: &PathBuf) -> Result<Self> {
        let data = read_to_string(path)?;
        let mut nw: Self = toml::from_str(&data)?;
        nw.path = path
            .parent()
            .ok_or(anyhow!("Couldnt get parent dir"))?
            .to_path_buf();
        Ok(nw)
    }

    pub fn save(&self) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(self.path.join("network.toml"))?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }
}

impl Default for Lockfile {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
            server: Server::default(),
            files: vec![],
        }
    }
}

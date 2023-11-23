use std::{
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
    time::SystemTime,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::app::ResolvedFile;

use super::Downloadable;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Lockfile {
    #[serde(skip)]
    pub path: PathBuf,

    pub plugins: Vec<(Downloadable, ResolvedFile)>,
    pub mods: Vec<(Downloadable, ResolvedFile)>,

    pub files: Vec<BootstrappedFile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BootstrappedFile {
    pub path: PathBuf,
    pub date: SystemTime,
}

impl Lockfile {
    pub fn get_lockfile(output_dir: &Path) -> Result<Self> {
        if output_dir.join(".mcman.lock").exists() {
            Ok(Self::load_from(&output_dir.join(".mcman.lock"))?)
        } else {
            Ok(Self {
                path: output_dir.join(".mcman.lock"),
                ..Default::default()
            })
        }
    }

    pub fn load_from(path: &PathBuf) -> Result<Self> {
        let data = read_to_string(path)?;
        let mut nw: Self = serde_json::from_str(&data)?;
        nw.path = path.to_owned();
        Ok(nw)
    }

    pub fn save(&self) -> Result<()> {
        let cfg_str = serde_json::to_string_pretty(&self)?;
        let mut f = File::create(&self.path)?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }
}

impl Default for Lockfile {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./.mcman.lock"),
            plugins: vec![],
            mods: vec![],
            files: vec![],
        }
    }
}

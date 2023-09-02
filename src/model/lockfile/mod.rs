use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf, time::SystemTime, collections::HashMap,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{Downloadable, ClientSideMod, World, Server};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Lockfile {
    #[serde(skip)]
    pub path: PathBuf,

    pub plugins: Vec<(String, Downloadable)>,
    pub mods: Vec<(String, Downloadable)>,
    pub clientsidemods: Vec<(String, ClientSideMod)>,
    pub worlds: HashMap<String, World>,

    pub files: Vec<BootstrappedFile>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BootstrappedFile {
    pub path: PathBuf,
    pub date: SystemTime,
}

#[derive(Debug)]
pub enum Change<T> {
    Added(T),
    Removed(T),
}

impl<T> Change<T> {
    pub fn inner(&self) -> &T {
        match self {
            Self::Added(t) | Self::Removed(t) => t
        }
    }
}

#[derive(Debug, Default)]
pub struct Changes {
    pub plugins: Vec<Change<(String, Downloadable)>>,
    pub mods: Vec<Change<(String, Downloadable)>>,
    // datapacks, etc...
}

impl Lockfile {
    pub fn get_lockfile(output_dir: &PathBuf) -> Result<Self> {
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

    pub fn get_changes(
        &self,
        server: &Server,
    ) -> Changes {
        let mut changes = Changes::default();

        // plugins

        let server_plugins: HashMap<Downloadable, String> = HashMap::from_iter(server.plugins
            .iter()
            .map(|p| (p.clone(), String::new())));

        let lockfile_plugins: HashMap<Downloadable, String> = HashMap::from_iter(self.plugins
            .iter().map(|(s, p)| (p.clone(), s.clone())));

        for added_plugin in server_plugins.keys().filter(|p| !lockfile_plugins.contains_key(p.to_owned())) {
            changes.plugins.push(Change::Added((String::new(), added_plugin.to_owned().clone())));
        }

        for removed_plugin in lockfile_plugins.keys().filter(|p| !server_plugins.contains_key(p.to_owned())) {
            let filename = lockfile_plugins[removed_plugin].clone();
            changes.plugins.push(Change::Removed((filename, removed_plugin.to_owned().clone())));
        }

        // mods

        let server_mods: HashMap<Downloadable, String> = HashMap::from_iter(server.mods
            .iter()
            .map(|p| (p.clone(), String::new())));

        let lockfile_mods: HashMap<Downloadable, String> = HashMap::from_iter(self.mods
            .iter().map(|(s, p)| (p.clone(), s.clone())));

        for added_mod in server_mods.keys().filter(|p| !lockfile_mods.contains_key(p.to_owned())) {
            changes.mods.push(Change::Added((String::new(), added_mod.to_owned().clone())));
        }

        for removed_mod in lockfile_mods.keys().filter(|p| !server_mods.contains_key(p.to_owned())) {
            let filename = lockfile_mods[removed_mod].clone();
            changes.mods.push(Change::Removed((filename, removed_mod.to_owned().clone())));
        }

        changes
    }
}

impl Default for Lockfile {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./.mcman.lock"),
            plugins: vec![],
            mods: vec![],
            worlds: HashMap::new(),
            clientsidemods: vec![],
            files: vec![],
        }
    }
}

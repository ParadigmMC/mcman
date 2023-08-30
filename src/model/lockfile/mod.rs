use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf, time::SystemTime, collections::{HashMap, HashSet},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{Downloadable, ClientSideMod, World, Server};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Lockfile {
    #[serde(skip)]
    pub path: PathBuf,

    pub plugins: Vec<Downloadable>,
    pub mods: Vec<Downloadable>,
    pub clientsidemods: Vec<ClientSideMod>,
    pub worlds: HashMap<String, World>,

    pub files: Vec<BootstrappedFile>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BootstrappedFile {
    pub path: PathBuf,
    pub date: SystemTime,
}

#[derive(Debug)]
pub enum Change {
    Added(ChangeType),
    Removed(ChangeType),
}

#[derive(Debug)]
pub enum ChangeType {
    Plugin(Downloadable),
    Mod(Downloadable),
    //ClientSideMod(ClientSideMod),
    //World(String),
    //Datapack(String, Downloadable),
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
        let mut nw: Self = toml::from_str(&data)?;
        nw.path = path.to_owned();
        Ok(nw)
    }

    pub fn save(&self) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(&self.path)?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }

    pub fn get_changes(
        &self,
        server: &Server,
    ) -> Vec<Change> {
        let mut changes = vec![];

        for pl in HashSet::<&Downloadable>::from_iter(&server.plugins)
            .difference(&HashSet::from_iter(&self.plugins)) {
            changes.push(Change::Added(ChangeType::Plugin(pl.clone().clone())));
        }

        for pl in HashSet::<&Downloadable>::from_iter(&self.plugins)
            .difference(&HashSet::from_iter(&server.plugins)) {
            changes.push(Change::Removed(ChangeType::Plugin(pl.clone().clone())));
        }

        for m in HashSet::<&Downloadable>::from_iter(&server.mods)
            .difference(&HashSet::from_iter(&self.mods)) {
            changes.push(Change::Added(ChangeType::Mod(m.clone().clone())));
        }

        for m in HashSet::<&Downloadable>::from_iter(&self.mods)
            .difference(&HashSet::from_iter(&server.mods)) {
            changes.push(Change::Removed(ChangeType::Mod(m.clone().clone())));
        }

        // todo: finish all

        changes
    }

    pub fn update(&mut self, server: &Server) {
        self.plugins = server.plugins.clone();
        self.mods = server.mods.clone();
        self.clientsidemods = server.clientsidemods.clone();
        self.worlds = server.worlds.clone();
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

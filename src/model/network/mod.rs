use std::{
    collections::HashMap,
    env,
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{ClientSideMod, Downloadable};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Network {
    #[serde(skip)]
    pub path: PathBuf,

    pub name: String,
    pub proxy: String,
    pub port: u16,
    pub variables: HashMap<String, String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<Downloadable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mods: Vec<Downloadable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub clientsidemods: Vec<ClientSideMod>,
}

impl Network {
    pub fn load() -> Result<Option<Self>> {
        let mut path = env::current_dir()?;
        let file = Path::new("network.toml");

        let found_path = loop {
            path.push(file);

            if path.is_file() {
                break path;
            }

            if !(path.pop() && path.pop()) {
                return Ok(None);
            }
        };

        Ok(Some(Self::load_from(&found_path)?))
    }

    pub fn load_from(path: &PathBuf) -> Result<Self> {
        let data = read_to_string(path)?;
        let mut nw: Self = toml::from_str(&data)?;
        nw.path = path.clone();
        Ok(nw)
    }

    pub fn save(&self) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(&self.path)?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }
}

impl Default for Network {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./network.toml"),
            name: String::new(),
            proxy: "proxy".to_owned(),
            port: 25565,
            variables: HashMap::new(),
            plugins: vec![],
            mods: vec![],
            clientsidemods: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct ServerEntry {
    pub port: u16,
}

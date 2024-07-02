use std::{
    collections::HashMap,
    env,
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{Downloadable, Hook, MarkdownOptions};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Network {
    #[serde(skip)]
    pub path: PathBuf,

    pub name: String,
    pub proxy: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub proxy_groups: Vec<String>,
    pub port: u16,
    pub servers: HashMap<String, ServerEntry>,
    pub variables: HashMap<String, String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "MarkdownOptions::is_empty")]
    pub markdown: MarkdownOptions,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub hooks: HashMap<String, Hook>,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub groups: HashMap<String, Group>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct ServerEntry {
    pub port: u16,
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Group {
    #[serde(default)]
    pub variables: HashMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<Downloadable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mods: Vec<Downloadable>,
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
        nw.path = path.parent().unwrap().to_path_buf();
        Ok(nw)
    }

    pub fn save(&self) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(self.path.join("network.toml"))?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }

    pub fn next_port(&self) -> u16 {
        let mut port = 25565;

        let mut taken = vec![self.port];
        for serv in self.servers.values() {
            taken.push(serv.port);
        }

        while taken.contains(&port) {
            port += 1;
        }

        port
    }
}

impl Default for Network {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
            name: String::new(),
            proxy: "proxy".to_owned(),
            proxy_groups: vec![],
            port: 25565,
            servers: HashMap::new(),
            variables: HashMap::new(),
            markdown: MarkdownOptions::default(),
            hooks: HashMap::new(),
            groups: HashMap::new(),
        }
    }
}

use std::{
    collections::HashMap,
    env,
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::sources::modrinth;

use super::{ClientSideMod, Downloadable, ServerLauncher, ServerType, SoftwareType, World};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct MarkdownOptions {
    pub files: Vec<String>,
    pub auto_update: bool,
}

impl MarkdownOptions {
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct Server {
    #[serde(skip)]
    pub path: PathBuf,

    pub name: String,
    pub mc_version: String, // TODO: version type for comparing
    #[serde(with = "super::servertype::parse")]
    pub jar: ServerType,
    pub variables: HashMap<String, String>,
    pub launcher: ServerLauncher,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<Downloadable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mods: Vec<Downloadable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub clientsidemods: Vec<ClientSideMod>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub worlds: HashMap<String, World>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MarkdownOptions::is_empty")]
    pub markdown: MarkdownOptions,
    #[serde(default)]
    pub options: ServerOptions,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ServerOptions {
    pub upload_to_mclogs: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bootstrap_exts: Vec<String>,
}

impl Server {
    pub fn load() -> Result<Self> {
        let mut path = env::current_dir()?;
        let file = Path::new("server.toml");

        let found_path = loop {
            path.push(file);

            if path.is_file() {
                break path;
            }

            if !(path.pop() && path.pop()) {
                bail!("Couldn't find server.toml - use `mcman init` to create one?");
            }
        };

        Self::load_from(&found_path)
    }

    pub fn load_from(path: &PathBuf) -> Result<Self> {
        let data = read_to_string(path)?;
        let mut serv: Self = toml::from_str(&data)?;
        serv.path = path
            .parent()
            .ok_or(anyhow!("Couldnt get parent dir"))?
            .canonicalize()?;
        Ok(serv)
    }

    pub fn save(&self) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(self.path.join("server.toml"))?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn format(&self, str: &str) -> String {
        mcapi::dollar_repl(str, |key| match key {
            "mcver" | "mcversion" | "SERVER_VERSION" => Some(self.mc_version.clone()),
            "SERVER_NAME" => Some(self.name.clone()),
            k => self.variables.get(k).cloned(),
        })
    }

    pub fn fill_from_map(&mut self, map: &HashMap<String, String>) {
        if let Some(v) = map.get("minecraft") {
            self.mc_version = v.clone();
        }

        if let Some(v) = map.get("forge") {
            self.jar = ServerType::Forge { loader: v.clone() }
        }

        if let Some(v) = map.get("neoforge") {
            self.jar = ServerType::NeoForge { loader: v.clone() }
        }

        if let Some(v) = map.get("fabric-loader").or(map.get("fabric")) {
            self.jar = ServerType::Fabric {
                loader: v.clone(),
                installer: "latest".to_owned(),
            }
        }

        if let Some(v) = map.get("quilt-loader").or(map.get("quilt")) {
            self.jar = ServerType::Quilt {
                loader: v.clone(),
                installer: "latest".to_owned(),
            }
        }
    }

    pub fn to_map(&self, include_loader: bool) -> HashMap<String, String> {
        let mut map = HashMap::from([("minecraft".to_owned(), self.mc_version.clone())]);

        let l = if include_loader { "-loader" } else { "" };

        if let Some((k, v)) = match &self.jar {
            ServerType::Quilt { loader, .. } => Some((format!("quilt{l}"), loader.clone())),
            ServerType::Fabric { loader, .. } => Some((format!("fabric{l}"), loader.clone())),
            ServerType::Forge { loader, .. } => Some(("forge".to_owned(), loader.clone())),
            ServerType::NeoForge { loader, .. } => Some(("neoforge".to_owned(), loader.clone())),
            _ => None,
        } {
            map.insert(k, v);
        }

        map
    }

    // TODO: move to ModrinthAPI
    pub fn filter_modrinth_versions(
        &self,
        list: &[modrinth::ModrinthVersion],
    ) -> Vec<modrinth::ModrinthVersion> {
        let is_proxy = self.jar.get_software_type() == SoftwareType::Proxy;
        let is_vanilla = matches!(self.jar, ServerType::Vanilla {});

        let mcver = &self.mc_version;
        let loader = self.jar.get_modrinth_name();

        list.iter()
            .filter(|v| is_proxy || v.game_versions.contains(mcver))
            .filter(|v| {
                if let Some(n) = &loader {
                    v.loaders
                        .iter()
                        .any(|l| l == "datapack" || l == n || (l == "fabric" && n == "quilt"))
                } else if is_vanilla {
                    v.loaders.contains(&"datapack".to_owned())
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }
}

impl Default for Server {
    fn default() -> Self {
        let mut vars = HashMap::new();
        vars.insert("PORT".to_owned(), "25565".to_owned());
        Self {
            path: PathBuf::from("."),
            name: String::new(),
            mc_version: "latest".to_owned(),
            jar: ServerType::Vanilla {},
            variables: vars,
            launcher: ServerLauncher::default(),
            plugins: vec![],
            mods: vec![],
            clientsidemods: vec![],
            worlds: HashMap::new(),
            markdown: MarkdownOptions::default(),
            options: ServerOptions::default(),
        }
    }
}

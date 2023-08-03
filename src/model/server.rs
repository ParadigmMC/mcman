use std::{
    collections::HashMap,
    env,
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::commands;

use super::{ClientSideMod, Downloadable, ServerLauncher, ServerType, World};

#[derive(Debug, Default, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Server {
    #[serde(skip)]
    pub path: PathBuf,

    pub name: String,
    pub mc_version: String, // TODO: version type for comparing
    #[serde(with = "super::servertype")]
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
            .to_path_buf();
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
        mcapi::dollar_repl(str, |key| {
            match key {
                "mcver" | "mcversion" => Some(self.mc_version.clone()),
                // Maybe also allow self.variables? idk
                _ => None,
            }
        })
    }

    pub async fn refresh_markdown(&self, http_client: &reqwest::Client) -> Result<()> {
        if self.markdown.auto_update {
            commands::markdown::update_files(http_client, self)
                .await
                .context("updating markdown files")
        } else {
            Ok(())
        }
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
        }
    }
}

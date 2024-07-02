use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::{mrpack::resolve_mrpack_addons, packwiz::resolve_packwiz_addons, Addon, AddonListFile, ModpackType};
use crate::api::{app::App, models::ModpackSource, utils::read_toml};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Source {
    File {
        path: String,
    },

    Folder {
        path: String,
    },

    Modpack {
        #[serde(flatten)]
        modpack: ModpackSource,
    },
}

impl Source {
    pub fn source_name(&self) -> &'static str {
        match self {
            Source::File { .. } => "File",
            Source::Folder { .. } => "Folder",
            Source::Modpack { .. } => "Modpack",
        }
    }

    pub fn source_accessor(&self) -> Result<String> {
        match self {
            Source::File { path } => Ok(path.clone()),
            Source::Folder { path } => Ok(path.clone()),
            Source::Modpack { modpack } => Ok(modpack.accessor()?.to_string()),
        }
    }

    pub fn modpack_type(&self) -> Option<ModpackType> {
        match self {
            Source::Modpack { modpack } => Some(modpack.modpack_type()),
            _ => None,
        }
    }

    pub async fn resolve_addons(&self, app: &App) -> Result<Vec<Addon>> {
        match self {
            Source::File { path } => {
                let file: AddonListFile = read_toml(&PathBuf::from(if path.ends_with(".toml") {
                    path.clone()
                } else {
                    format!("{path}.toml")
                })).with_context(|| format!("Source: File => {path}"))?;
                Ok(file.flatten())
            },

            Source::Folder { .. } => Ok(vec![]),

            Source::Modpack { modpack } => {
                let accessor = modpack.accessor()?;
                match modpack.modpack_type() {
                    ModpackType::MRPack => resolve_mrpack_addons(app, accessor).await,
                    ModpackType::Packwiz => resolve_packwiz_addons(app, accessor).await,
                    ModpackType::Unsup => todo!(),
                }.with_context(|| format!("Source: Modpack/{} => {}", modpack.modpack_type().to_string(), modpack.accessor().unwrap().to_string()))
            }
        }
    }
}

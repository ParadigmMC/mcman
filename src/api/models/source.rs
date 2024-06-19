use std::path::PathBuf;

use anyhow::Result;
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
    pub async fn resolve_addons(&self, app: &App) -> Result<Vec<Addon>> {
        match self {
            Source::File { path } => {
                let file: AddonListFile = read_toml(&PathBuf::from(if path.ends_with(".toml") {
                    path.clone()
                } else {
                    format!("{path}.toml")
                }))?;
                Ok(file.flatten())
            },

            Source::Folder { path } => Ok(vec![]),

            Source::Modpack { modpack } => {
                let accessor = modpack.accessor()?;
                match modpack.modpack_type() {
                    ModpackType::MRPack => resolve_mrpack_addons(app, accessor).await,
                    ModpackType::Packwiz => resolve_packwiz_addons(app, accessor).await,
                    ModpackType::Unsup => todo!(),
                }
            }
        }
    }
}

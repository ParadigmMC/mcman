use std::path::Path;

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{mrpack::resolve_mrpack_addons, packwiz::resolve_packwiz_addons, Addon, AddonListFile, ModpackType};
use crate::api::{app::App, models::ModpackSource, utils::{accessor::Accessor, fs::with_extension_if_none, toml::read_toml}};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Source {
    #[serde(flatten)]
    pub source_type: SourceType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SourceType {
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
        match self.source_type {
            SourceType::File { .. } => "File",
            SourceType::Folder { .. } => "Folder",
            SourceType::Modpack { .. } => "Modpack",
        }
    }

    pub fn accessor(&self, relative_to: &Path) -> Result<Accessor> {
        match &self.source_type {
            SourceType::File { path } => Ok(Accessor::Local(path.into())),
            SourceType::Folder { path } => Ok(Accessor::Local(path.into())),
            SourceType::Modpack { modpack } => modpack.accessor(relative_to),
        }
    }

    pub fn modpack_type(&self) -> Option<ModpackType> {
        match &self.source_type {
            SourceType::Modpack { modpack } => Some(modpack.modpack_type()),
            _ => None,
        }
    }

    pub async fn resolve_addons(&self, app: &App, relative_to: &Path) -> Result<Vec<Addon>> {
        match &self.source_type {
            SourceType::File { path } => {
                let file: AddonListFile = read_toml(&with_extension_if_none(&relative_to.join(path), "toml")).with_context(|| format!("Source: File => {path}"))?;
                Ok(file.flatten())
            },

            SourceType::Folder { .. } => Ok(vec![]),

            SourceType::Modpack { modpack } => {
                let accessor = modpack.accessor(relative_to)?;
                match modpack.modpack_type() {
                    ModpackType::MRPack => resolve_mrpack_addons(app, accessor).await,
                    ModpackType::Packwiz => resolve_packwiz_addons(app, accessor).await,
                    ModpackType::Unsup => todo!(),
                }.with_context(|| format!("Source: Modpack/{} => {}", modpack.modpack_type().to_string(), modpack.accessor(relative_to).unwrap().to_string()))
            }
        }
    }
}

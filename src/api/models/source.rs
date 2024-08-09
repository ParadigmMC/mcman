use std::{path::Path, str::FromStr};

use anyhow::{anyhow, bail, Context, Result};
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
        modpack_source: ModpackSource,        
        modpack_type: ModpackType,
    },
}

impl FromStr for SourceType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (ty, val) = s.split_once(':').ok_or(anyhow!("Source identifier are in the format of '<type>:<path or url>'"))?;
        
        Ok(match ty {
            "file" | "f" => SourceType::File { path: val.into() },
            "packwiz" | "pw" => SourceType::Modpack { modpack_source: ModpackSource::from_str(val)?, modpack_type: ModpackType::Packwiz },
            "mrpack" => SourceType::Modpack { modpack_source: ModpackSource::from_str(val)?, modpack_type: ModpackType::MRPack },
            _ => bail!("Unknown source identifier type: {ty}"),
        })
    }
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
            SourceType::File { path } => Ok(Accessor::Local(relative_to.join(path).canonicalize()
                .with_context(|| format!("Resolving path: {:?}", relative_to.join(path)))?)),
            SourceType::Folder { path } => Ok(Accessor::Local(relative_to.join(path).canonicalize()
                .with_context(|| format!("Resolving path: {:?}", relative_to.join(path)))?)),
            SourceType::Modpack { modpack_source: modpack, .. } => modpack.accessor(relative_to)
                .with_context(|| "Getting Modpack Accessor"),
        }
    }

    pub fn modpack_type(&self) -> Option<ModpackType> {
        match &self.source_type {
            SourceType::Modpack { modpack_type, .. } => Some(modpack_type.clone()),
            _ => None,
        }
    }

    pub async fn resolve_addons(&self, app: &App, relative_to: &Path) -> Result<Vec<Addon>> {
        match &self.source_type {
            SourceType::File { path } => {
                let path = with_extension_if_none(&relative_to.join(path), "toml");
                log::info!("Source: File => {path:?}");
                let file: AddonListFile = read_toml(&path).with_context(|| format!("Source: File => {path:?}"))?;
                Ok(file.flatten())
            },

            SourceType::Folder { .. } => Ok(vec![]),

            SourceType::Modpack { modpack_source: modpack, modpack_type } => {
                let accessor = modpack.accessor(relative_to)?;
                match modpack_type {
                    ModpackType::MRPack => resolve_mrpack_addons(app, accessor).await,
                    ModpackType::Packwiz => resolve_packwiz_addons(app, accessor).await,
                    ModpackType::Unsup => todo!(),
                }.with_context(|| format!("Source: Modpack/{} => {}", modpack_type.to_string(), modpack.accessor(relative_to).unwrap().to_string()))
            }
        }
    }
}

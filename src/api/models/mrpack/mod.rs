use anyhow::{anyhow, Result};

mod mrpack_file;
mod mrpack_index;

pub use mrpack_file::*;
pub use mrpack_index::*;

pub const MRPACK_INDEX_FILE: &str = "modrinth.index.json";

use crate::api::{app::App, utils::accessor::Accessor};

use super::{server::ServerJar, Addon, AddonTarget, AddonType};

pub async fn resolve_mrpack_serverjar(app: &App, mut accessor: Accessor) -> Result<ServerJar> {
    let index: MRPackIndex = accessor.json(app, MRPACK_INDEX_FILE).await?;

    ServerJar::try_from(index.dependencies.clone())
}

pub async fn resolve_mrpack_addons(app: &App, mut accessor: Accessor) -> Result<Vec<Addon>> {
    let mut addons = vec![];

    let index: MRPackIndex = accessor.json(app, MRPACK_INDEX_FILE).await?;

    for file in index.files {
        addons.push(file.into_addon().await?);
    }

    Ok(addons)
}

impl MRPackFile {
    pub async fn into_addon(&self) -> Result<Addon> {
        Ok(Addon {
            environment: self.env.as_ref().map(|e| e.clone().into()),
            addon_type: AddonType::Url {
                url: self
                    .downloads
                    .first()
                    .ok_or(anyhow!("No downloads"))?
                    .clone(),
            },
            target: AddonTarget::from_path(&self.path),
        })
    }
}

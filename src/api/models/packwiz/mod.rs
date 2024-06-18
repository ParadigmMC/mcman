mod packwiz_mod;
mod packwiz_pack;

use std::path::Path;

use anyhow::{bail, Result};
pub use packwiz_mod::*;
pub use packwiz_pack::*;

use crate::api::{app::App, models::AddonType, utils::accessor::Accessor};

pub static PACK_TOML: &str = "pack.toml";

use super::{Addon, AddonTarget};

pub async fn resolve_packwiz_addons(app: &App, mut accessor: Accessor) -> Result<Vec<Addon>> {
    let mut addons = vec![];

    let pack: PackwizPack = accessor.toml(app, PACK_TOML).await?;

    let index: PackwizPackIndex = accessor.toml(app, &pack.index.path).await?;

    for file in index.files.iter().filter(|f| f.metafile) {
        let pw_mod: PackwizMod = accessor.toml(app, &file.path).await?;

        let target = AddonTarget::from_path(&file.path);

        addons.push(pw_mod.into_addon(app, target).await?);
    }

    Ok(addons)
}

impl PackwizMod {
    pub async fn into_addon(&self, app: &App, target: AddonTarget) -> Result<Addon> {
        let addon_type = if let Some(update) = &self.update {
            match update {
                PackwizModUpdate::Modrinth { mod_id, version } => AddonType::Modrinth {
                    id: mod_id.clone(),
                    version: version.clone(),
                },
                PackwizModUpdate::Curseforge {
                    file_id,
                    project_id,
                } => AddonType::Curseforge {
                    id: project_id.to_string(),
                    version: file_id.to_string(),
                },
            }
        } else {
            let Some(url) = &self.download.url else {
                bail!("Packwiz mod {self:?} has neither `url` or Curseforge metadata");
            };

            AddonType::Url { url: url.clone() }
        };

        let addon = Addon {
            environment: Some(self.side),
            addon_type,
            target,
        };

        Ok(addon)
    }
}

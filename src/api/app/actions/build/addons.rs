use std::{collections::HashSet, path::Path, sync::Arc};

use anyhow::{Context, Result};
use futures::{stream, StreamExt, TryStreamExt};

use crate::api::{app::App, models::Addon, step::Step};

impl App {
    /// Installs new addons and removes old removed addons
    pub async fn action_install_addons(self: Arc<Self>, base: &Path) -> Result<()> {
        let addons = self.collect_addons().await?;
        let base = Arc::new(base.to_owned());

        let (addons_to_add, addons_to_remove): (Vec<Addon>, Vec<Addon>) = if let Some(lockfile) = &*self.existing_lockfile.read().await {
            let mut old = HashSet::new();
            old.extend(lockfile.addons.clone());

            let mut new = HashSet::new();
            new.extend(addons);

            (new.difference(&old).map(ToOwned::to_owned).collect(), old.difference(&new).map(ToOwned::to_owned).collect())
        } else {
            (addons, vec![])
        };

        for addon in &addons_to_remove {
            self.clone().action_remove_addon(&base, addon).await?;
        }

        stream::iter(addons_to_add).map(Ok).try_for_each_concurrent(
            Some(20),
            move |addon| {
                let app = self.clone();
                let base = base.clone();
                async move {
                    app.action_install_addon(&base, &addon).await
                        .with_context(|| format!("{addon:#?}"))
                }
            }
        ).await?;

        Ok(())
    }

    /// Installs a single addon
    pub async fn action_install_addon(self: Arc<Self>, base: &Path, addon: &Addon) -> Result<()> {
        let steps = addon.resolve_steps(&self).await?;
        let dir = base.join(addon.target.as_str());
        self.execute_steps(&dir, &steps).await?;
        Ok(())
    }

    /// Removes a single addon
    pub async fn action_remove_addon(self: Arc<Self>, base: &Path, addon: &Addon) -> Result<()> {
        let steps = addon.resolve_steps(&self).await?;
        let dir = base.join(addon.target.as_str());

        // TODO
        
        if let Some(meta) = steps.iter().find_map(|x| match x {
            Step::CacheCheck(meta) => Some(meta),
            Step::Download { metadata, .. } => Some(metadata),
            _ => None,
        }) {
            tokio::fs::remove_file(dir.join(&meta.filename)).await?;
        } else {
            log::error!("Couldn't remove addon: {addon:#?}");
        }

        Ok(())
    }
}

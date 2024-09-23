use std::{collections::HashSet, path::Path, sync::Arc};

use anyhow::{Context, Result};
use futures::{stream, StreamExt, TryStreamExt};

use crate::api::{app::App, models::Addon};

impl App {
    /// Installs new addons and removes old removed addons
    pub async fn action_install_addons(self: Arc<Self>, base: &Path) -> Result<()> {
        let addons = self.collect_addons().await?;
        let base = Arc::new(base.to_owned());

        println!("Found {} addons", addons.len());

        let (addons_to_add, addons_to_remove): (Vec<Addon>, Vec<Addon>) =
            if let Some(lockfile) = &*self.existing_lockfile.read().await {
                let mut old = HashSet::new();
                old.extend(lockfile.addons.clone());

                let mut new = HashSet::new();
                new.extend(addons);

                (
                    new.difference(&old).map(ToOwned::to_owned).collect(),
                    old.difference(&new).map(ToOwned::to_owned).collect(),
                )
            } else {
                (addons, vec![])
            };

        println!(
            "Installing {} addons, removing {} addons",
            addons_to_add.len(),
            addons_to_remove.len()
        );

        for addon in &addons_to_remove {
            self.clone().action_remove_addon(&base, addon).await?;
        }

        stream::iter(addons_to_add)
            .map(Ok)
            .try_for_each_concurrent(Some(20), move |addon| {
                let app = self.clone();
                let base = base.clone();
                async move {
                    let x = app
                        .clone()
                        .action_install_addon(&base, &addon)
                        .await
                        .with_context(|| format!("{addon:#?}"));

                    if x.is_ok() {
                        app.add_addon_to_lockfile(addon).await;
                    }

                    x
                }
            })
            .await?;

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
        let steps = addon.resolve_remove_steps(&self).await?;
        let dir = base.join(addon.target.as_str());
        self.execute_steps(&dir, &steps).await?;
        Ok(())
    }
}

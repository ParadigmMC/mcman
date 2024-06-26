use std::{path::Path, sync::Arc};
use futures::stream::{self, StreamExt, TryStreamExt};

use anyhow::Result;

use crate::api::{app::App, models::{Addon, Environment}};

impl App {
    pub async fn action_install_jar(&self, base: &Path) -> Result<()> {
        if let Some(jar) = self.server.read().await.as_ref().map(|(_, server)| {
            server.jar.clone()
        }).flatten() {
            println!("Installing server jar");

            let steps = jar.resolve_steps(&self, Environment::Server).await?;

            println!("{steps:#?}");

            self.execute_steps(base, &steps).await?;
        }

        Ok(())
    }

    pub async fn action_install_addons(self: Arc<Self>, base: &Path) -> Result<()> {
        let addons = self.collect_addons().await?;
        let base = Arc::new(base.to_owned());

        const MAX_CONCURRENT_TASKS: usize = 20;

        stream::iter(addons).map(Ok).try_for_each_concurrent(
            Some(MAX_CONCURRENT_TASKS),
            move |addon| {
                let app = self.clone();
                let base = base.clone();
                async move {
                    app.action_install_addon(&base, &addon).await
                }
            }
        ).await?;

        Ok(())
    }

    pub async fn action_install_addon(self: Arc<Self>, base: &Path, addon: &Addon) -> Result<()> {
        let steps = addon.resolve_steps(&self).await?;
        let dir = base.join(addon.target.as_str());
        self.execute_steps(&dir, &steps).await?;
        Ok(())
    }
}

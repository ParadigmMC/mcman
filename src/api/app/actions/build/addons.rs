use std::{path::Path, sync::Arc};

use anyhow::{Context, Result};
use futures::{stream, StreamExt, TryStreamExt};

use crate::api::{app::App, models::Addon};

impl App {
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
                        .with_context(|| format!("{addon:#?}"))
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

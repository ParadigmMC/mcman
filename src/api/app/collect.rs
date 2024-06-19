use anyhow::Result;

use crate::api::models::{Addon, Source};

use super::App;

impl App {
    pub async fn collect_sources(&self) -> Result<Vec<Source>> {
        let mut sources = vec![];

        if let Some(lock) = &self.server {
            let server = lock.read().await;

            sources.append(&mut server.sources.clone());
        }

        Ok(sources)
    }

    pub async fn collect_addons(&self) -> Result<Vec<Addon>> {
        let mut addons = vec![];

        for source in self.collect_sources().await? {
            addons.append(&mut source.resolve_addons(&self).await?);
        }

        Ok(addons)
    }
}

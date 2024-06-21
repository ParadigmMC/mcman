use anyhow::Result;

use crate::api::models::{Addon, Source};

use super::App;

impl App {
    pub async fn collect_sources(&self) -> Result<Vec<Source>> {
        let mut sources = vec![];

        if let Some((_, server)) = &*self.server.read().await {
            sources.extend_from_slice(&server.sources);
        }

        Ok(sources)
    }

    pub async fn collect_addons(&self) -> Result<Vec<Addon>> {
        let mut addons = vec![];

        for source in self.collect_sources().await? {
            addons.extend_from_slice(&source.resolve_addons(&self).await?);
        }

        Ok(addons)
    }
}

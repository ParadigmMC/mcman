use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::api::app::App;

pub struct McmanMetaAPI<'a>(pub &'a App);

impl<'a> McmanMetaAPI<'a> {
    pub async fn get(&self, folder: &str, path: &str) -> Result<String> {
        Ok(self
            .0
            .http_get(format!(
                "{}/{folder}/{path}",
                self.0.options.api_urls.mcman_meta
            ))
            .await?
            .text()
            .await?)
    }

    pub async fn ls(&self, folder: &str) -> Result<Vec<String>> {
        Ok(self
            .get(folder, "ls")
            .await?
            .split('\n')
            .map(ToOwned::to_owned)
            .collect())
    }
}

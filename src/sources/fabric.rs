use std::collections::HashMap;

use anyhow::{anyhow, Result};
use mcapi::fabric::{FabricLoader, FabricInstaller, FABRIC_META_URL};

use crate::{App, FileSource, CacheStrategy};

pub struct FabricAPI<'a>(pub &'a App);

impl<'a> FabricAPI<'a> {
    pub async fn fetch_loaders(&self) -> Result<Vec<FabricLoader>> {
        Ok(mcapi::fabric::fetch_loaders(&self.0.http_client).await?)
    }

    pub async fn fetch_latest_loader(&self) -> Result<String> {
        Ok(self.fetch_loaders().await?.first().ok_or(anyhow!("No fabric loaders???"))?.version.clone())
    }

    pub async fn fetch_installers(&self) -> Result<Vec<FabricInstaller>> {
        Ok(mcapi::fabric::fetch_installers(&self.0.http_client).await?)
    }

    pub async fn fetch_latest_installer(&self) -> Result<String> {
        Ok(self.fetch_installers().await?.first().ok_or(anyhow!("No fabric installers???"))?.version.clone())
    }

    pub async fn resolve_source(&self, loader: &str, installer: &str) -> Result<FileSource> {
        let loader = match loader {
            "latest" => self.fetch_latest_loader().await?,
            id => id.to_owned(),
        };

        let installer = match installer {
            "latest" => self.fetch_latest_installer().await?,
            id => id.to_owned(),
        };

        let cached_file_path = format!("fabric-server-{}-{installer}-{loader}.jar", self.0.mc_version());

        if self.0.has_in_cache("fabric", &cached_file_path) {
            Ok(FileSource::Cached {
                path: self.0.get_cache("fabric").unwrap().0.join(&cached_file_path),
                filename: cached_file_path.clone(),
            })
        } else {
            Ok(FileSource::Download {
                url: format!(
                    "{FABRIC_META_URL}/v2/versions/loader/{}/{loader}/{installer}/server/jar",
                    self.0.mc_version()
                ),
                filename: cached_file_path.clone(),
                cache: if let Some(cache) = self.0.get_cache("fabric") {
                    CacheStrategy::File { path: cache.0.join(cached_file_path) }
                } else {
                    CacheStrategy::None
                },
                size: None,
                hashes: HashMap::new(),
            })
        }
    }
}

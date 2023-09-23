use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::{App, FileSource, CacheStrategy};

pub struct VanillaAPI<'a>(&'a App);

pub const CACHE_DIR: &str = "vanilla";

impl<'a> VanillaAPI<'a> {
    pub async fn fetch_latest_mcver(&self) -> Result<String> {
        Ok(mcapi::vanilla::fetch_version_manifest(&self.0.http_client).await?.latest.release)
    }

    pub async fn resolve_source(&self, version: &str) -> Result<FileSource> {
        let version_manifest = mcapi::vanilla::fetch_version_manifest(&self.0.http_client).await?;

        let version = match version {
            "latest" => version_manifest.fetch_latest_release(&self.0.http_client).await?,
            "latest-snapshot" => version_manifest.fetch_latest_snapshot(&self.0.http_client).await?,
            id => version_manifest.fetch(id, &self.0.http_client).await?,
        };

        let file = version.downloads
            .get(&mcapi::vanilla::DownloadType::Server)
            .ok_or(anyhow!(
                "version manifest doesn't include a server download"
            ))?;
        
        let cached_file_path = format!("server-{}.jar", version.id);

        let has_in_cache = self.0.has_in_cache(CACHE_DIR, &cached_file_path);

        if has_in_cache {
            Ok(FileSource::Cached {
                path: self.0.get_cache(CACHE_DIR).unwrap().0.join(&cached_file_path),
                filename: cached_file_path.clone(),
            })
        } else {
            Ok(FileSource::Download {
                url: file.url.clone(),
                filename: cached_file_path.clone(),
                cache: if let Some(cache) = self.0.get_cache(CACHE_DIR) {
                    CacheStrategy::File { path: cache.0.join(cached_file_path) }
                } else {
                    CacheStrategy::None
                },
                size: Some(file.size as i32),
                hashes: HashMap::from([
                    ("sha1".to_owned(), file.sha1.clone())
                ]),
            })
        }
    }
}

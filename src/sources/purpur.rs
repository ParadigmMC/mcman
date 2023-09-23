use std::collections::HashMap;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{App, FileSource, CacheStrategy};

pub struct PurpurAPI<'a>(&'a App);

pub const API_URL: &str = "https://api.purpurmc.org/v2/purpur";
pub const CACHE_DIR: &str = "purpur";

impl<'a> PurpurAPI<'a> {
    pub async fn fetch_api<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response: T = self.0.http_client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn fetch_versions(&self) -> Result<Vec<String>> {
        Ok(self.fetch_api::<PurpurMCResponse>(API_URL).await?.versions)
    }

    pub async fn fetch_builds(&self, version: &str) -> Result<PurpurMCBuilds> {
        Ok(self.fetch_api::<PurpurMCVersion>(&format!("{API_URL}/{version}?detailed=true")).await?.builds)
    }

    pub async fn fetch_build(&self, version: &str, build: &str) -> Result<PurpurMCBuild> {
        let builds = self.fetch_builds(version).await?;

        Ok(match build {
            "latest" => builds.latest.clone(),
            id => builds.all.iter().find(|b| b.build == id).ok_or(anyhow!("Cant find build '{id}' of Purpur {version}"))?.clone(),
        })
    }

    pub async fn resolve_source(&self, version: &str, build: &str) -> Result<FileSource> {
        let resolved_build = self.fetch_build(version, build).await?;
        
        let cached_file_path = format!("purpur-{version}-{}.jar", resolved_build.build);

        let has_in_cache = self.0.has_in_cache(CACHE_DIR, &cached_file_path);

        if has_in_cache {
            Ok(FileSource::Cached {
                path: self.0.get_cache(CACHE_DIR).unwrap().0.join(&cached_file_path),
                filename: cached_file_path.clone(),
            })
        } else {
            Ok(FileSource::Download {
                url: format!(
                    "{API_URL}/{version}/{}/download",
                    resolved_build.build
                ),
                filename: cached_file_path.clone(),
                cache: if let Some(cache) = self.0.get_cache(CACHE_DIR) {
                    CacheStrategy::File { path: cache.0.join(cached_file_path) }
                } else {
                    CacheStrategy::None
                },
                size: None,
                hashes: HashMap::from([
                    ("md5".to_owned(), resolved_build.md5)
                ]),
            })
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PurpurMCResponse {
    pub project: String,
    pub versions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PurpurMCVersion {
    pub builds: PurpurMCBuilds,
    pub project: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PurpurMCBuilds {
    pub latest: PurpurMCBuild,
    pub all: Vec<PurpurMCBuild>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PurpurMCBuild {
    pub project: String,
    pub version: String,
    pub build: String,
    pub md5: String,
}

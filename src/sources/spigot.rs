use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{App, FileSource, CacheStrategy};

#[derive(Debug, Deserialize, Serialize)]
struct SpigotResourceVersion {
    pub name: String,
    pub id: i32,
}



pub struct SpigotAPI<'a>(&'a App);

pub const API_URL: &str = "https://api.spiget.org/v2";
pub const CACHE_DIR: &str = "spiget";

impl<'a> SpigotAPI<'a> {
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

    pub fn get_resource_id(res: &str) -> &str {
        if let Some(i) = res.find('.') {
            if i < res.len() - 1 {
                return res.split_at(i + 1).1;
            }
        }
    
        res
    }

    pub async fn fetch_info(&self, id: &str) -> Result<(String, String)> {
        let json = self.fetch_api::<serde_json::Value>(&format!("{API_URL}/resources/{}", Self::get_resource_id(id))).await?;

        Ok((
            json["name"].as_str().unwrap().to_owned(),
            json["tag"].as_str().unwrap().to_owned(),
        ))
    }

    pub async fn fetch_versions(&self, id: &str) -> Result<Vec<SpigotVersion>> {
        self.fetch_api(&format!("{API_URL}/resources/{}/versions", Self::get_resource_id(id))).await
    }

    pub async fn fetch_version(&self, id: &str, version: &str) -> Result<SpigotVersion> {
        self.fetch_api(&format!("{API_URL}/resources/{}/versions/{version}", Self::get_resource_id(id))).await
    }

    pub async fn resolve_source(&self, id: &str, version: &str) -> Result<FileSource> {
        let resolved_version = self.fetch_version(id, version).await?;

        let filename = format!("spigot-{id}-{}.jar", resolved_version.name);
        let cached_file_path = format!("{id}/{}.jar", resolved_version.id);

        let has_in_cache = self.0.has_in_cache(CACHE_DIR, &cached_file_path);

        if has_in_cache {
            Ok(FileSource::Cached {
                path: self.0.get_cache(CACHE_DIR).unwrap().0.join(cached_file_path),
                filename: filename.clone(),
            })
        } else {
            Ok(FileSource::Download {
                url: format!(
                    "{API_URL}/resources/{}/versions/{version}/download",
                    Self::get_resource_id(id)
                ),
                filename,
                cache: if let Some(cache) = self.0.get_cache(CACHE_DIR) {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpigotVersion {
    pub uuid: String,
    pub name: String,
    pub resource: i64,
    pub id: i64,
}

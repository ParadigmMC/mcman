use std::{time::{Duration, SystemTime, UNIX_EPOCH}, collections::HashMap};

use anyhow::{anyhow, Result, Context};
use async_trait::async_trait;
use reqwest::{header::{HeaderMap, HeaderValue}, StatusCode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::time::sleep;

use crate::{App, FileSource, CacheStrategy};

pub trait GithubRequestExt {
    fn with_token(self, token: Option<String>) -> Self;
}

impl GithubRequestExt for reqwest::RequestBuilder {
    fn with_token(self, token: Option<String>) -> Self {
        if let Some(token) = token {
            self.bearer_auth(token)
        } else {
            self
        }
    }
}

#[async_trait]
pub trait GithubWaitRatelimit<T> {
    async fn wait_ratelimit(self) -> Result<T>;
}

#[async_trait]
impl GithubWaitRatelimit<reqwest::Response> for reqwest::Response {
    async fn wait_ratelimit(self) -> Result<Self> {
        let res = if let Some(h) = self.headers().get("x-ratelimit-remaining") {
            if String::from_utf8_lossy(h.as_bytes()) == "1" {
                let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                let ratelimit_reset =
                    String::from_utf8_lossy(self.headers()["x-ratelimit-reset"].as_bytes())
                        .parse::<u64>()?;
                let amount = ratelimit_reset - now;
                println!(" (!) Github ratelimit exceeded. sleeping for {amount} seconds...");
                sleep(Duration::from_secs(amount)).await;
            }
            self
        } else {
            self.error_for_status()?
        };
    
        Ok(res)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CachedData<T: Serialize> {
    pub data: T,
    pub last_modified: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubRelease {
    pub tag_name: String,
    pub name: String,
    pub assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubAsset {
    pub url: String,
    pub name: String,
    pub size: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubRepository {
    pub description: String
}

static CACHE_DIR: &str = "github";
static API_URL: &str = "https://api.github.com";

pub struct GithubAPI<'a>(&'a App);

impl<'a> GithubAPI<'a> {
    pub async fn fetch_api<T: DeserializeOwned + Clone + Serialize>(
        &self,
        url: String,
        cache_path: String
    ) -> Result<T> {
        let cached_data = if let Some(cache) = self.0.get_cache(CACHE_DIR) {
            if let Some(json) = cache.try_get_json::<CachedData<T>>(&cache_path)? {
                Some(json)
            } else {
                None
            }
        } else {
            None
        };

        let response = self.0.http_client
            .get(&url)
            .with_token(None) // TODO: token via App
            .headers(if let Some(cached_data) = &cached_data {
                let mut map = HeaderMap::new();
                map.insert("If-Modified-Since", HeaderValue::from_str(&cached_data.last_modified)?);
                map
            } else {
                HeaderMap::new()
            })
            .send()
            .await?;
        
        if response.status() == StatusCode::NOT_MODIFIED {
            Ok(cached_data.unwrap().data)
        } else {
            let last_modified = response.headers().get("Last-Modified").cloned();

            let json: T = response
                .error_for_status()?
                .wait_ratelimit()
                .await?
                .json()
                .await?;

            if let Some(last_modified) = last_modified {
                if let Some(cache) = self.0.get_cache(CACHE_DIR) {
                    cache.write_json(&cache_path, &CachedData {
                        last_modified: last_modified.to_str()?.to_owned(),
                        data: json.clone(),
                    }).context("Saving github api response to cache")?;
                }
            }

            Ok(json)
        }
    }

    pub async fn fetch_repo_description(&self, repo: &str) -> Result<String> {
        Ok(
            self.fetch_api::<GithubRepository>(
                format!("{API_URL}/repos/{repo}"),
                format!("{repo}/repository.json")
            ).await?.description
        )
    }

    pub async fn fetch_releases(&self, repo: &str) -> Result<Vec<GithubRelease>> {
        Ok(
            self.fetch_api::<Vec<GithubRelease>>(
                format!("{API_URL}/repos/{repo}/releases"),
                format!("{repo}/releases.json")
            ).await?
        )
    }

    pub async fn fetch_release(&self, repo: &str, release_tag: &str) -> Result<GithubRelease> {
        let releases = self.fetch_releases(repo).await?;

        let tag = release_tag.replace("${mcver}", &self.0.mc_version());
        let tag = tag.replace("${mcversion}", &self.0.mc_version());

        let release = match tag.as_str() {
            "latest" => releases.first(),
            tag => releases.iter()
                .find(|r| r.tag_name == tag)
                .or_else(|| releases.iter().find(|r| r.tag_name.contains(tag)))
        }.ok_or(anyhow!("Github release '{tag}' ('{release_tag}') not found on repository '{repo}'"))?;

        Ok(release.clone())
    }

    pub async fn fetch_asset(&self, repo: &str, release_tag: &str, asset_name: &str) -> Result<(GithubRelease, GithubAsset)> {
        let release = self.fetch_release(repo, release_tag).await?;

        let asset = match asset_name {
            "" | "first" | "any" => release.assets.first(),
            id => {
                let id = if id.contains('$') {
                    id.replace("${version}", &release.tag_name)
                        .replace("${tag}", &release.tag_name)
                        .replace("${release}", &release.tag_name)
                        .replace("${mcver}", &self.0.mc_version())
                        .replace("${mcversion}", &self.0.mc_version())
                } else {
                    id.to_owned()
                };
    
                release.assets
                    .iter()
                    .find(|a| id == a.name)
                    .or(release.assets.iter().find(|a| a.name.contains(&id)))
            }
        }
        .ok_or(anyhow!(
            "Github release asset '{asset_name}' on release '{}' ('{release_tag}') of repository '{repo}' not found",
            release.tag_name
        ))?.clone();
    
        Ok((release, asset))
    }

    pub async fn resolve_source(&self, repo: &str, release_tag: &str, asset_name: &str) -> Result<FileSource> {
        let (release, asset) = self.fetch_asset(repo, release_tag, asset_name).await?;
        
        let cached_file_path = format!("{repo}/releases/{}/{}", release.tag_name, asset.name);

        let has_in_cache = self.0.has_in_cache(CACHE_DIR, &cached_file_path);

        if has_in_cache {
            Ok(FileSource::Cached {
                path: self.0.get_cache(CACHE_DIR).unwrap().0.join(cached_file_path),
                filename: asset.name,
            })
        } else {
            Ok(FileSource::Download {
                url: format!(
                    "https://github.com/{repo}/releases/download/{}/{}",
                    release.tag_name, asset.name
                ),
                filename: asset.name,
                cache: if let Some(cache) = self.0.get_cache(CACHE_DIR) {
                    CacheStrategy::File { path: cache.0.join(cached_file_path) }
                } else {
                    CacheStrategy::None
                },
                size: Some(asset.size),
                hashes: HashMap::new(),
            })
        }
    }
}


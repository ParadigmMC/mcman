use anyhow::{anyhow, Context, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    StatusCode,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::api::{
    app::App,
    step::{CacheLocation, FileMeta, Step},
};

mod models;
pub use models::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CachedData<T: Serialize> {
    pub data: T,
    pub etag: String,
}

static CACHE_DIR: &str = "github";
static GITHUB_API_VERSION: &str = "2022-11-28";

pub struct GithubAPI<'a>(pub &'a App);

impl<'a> GithubAPI<'a> {
    pub async fn fetch_api<T: DeserializeOwned + Clone + Serialize>(
        &self,
        url: String,
        cache_path: String,
    ) -> Result<T> {
        let path = format!("{CACHE_DIR}/{cache_path}");
        let cached_data = self.0.cache.try_get_json::<CachedData<T>>(&path)?;

        let mut headers = HeaderMap::new();

        if let Some(token) = &self.0.options.github_token {
            headers.insert("Authorization", HeaderValue::from_str(token)?);
        }

        if let Some(cached_data) = &cached_data {
            headers.insert("if-none-match", HeaderValue::from_str(&cached_data.etag)?);
        }
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_str(GITHUB_API_VERSION)?,
        );

        let response = self
            .0
            .http_get_with(format!("{}/{url}", self.0.options.api_urls.github), |req| {
                req.headers(headers)
            })
            .await
            .with_context(|| format!("Github: HTTP GET /{url}"))?;

        if response.status() == StatusCode::NOT_MODIFIED {
            Ok(cached_data.unwrap().data)
        } else {
            let etag = response.headers().get("etag").cloned();

            let json: T = response
                .json()
                .await
                .with_context(|| format!("Github: JSON decoding: {url}"))?;

            if let Some(etag) = etag {
                self.0.cache.try_write_json(
                    &path,
                    &CachedData {
                        etag: etag.to_str()?.to_owned(),
                        data: json.clone(),
                    },
                )?;
            }

            Ok(json)
        }
    }

    pub async fn fetch_repo_description(&self, repo: &str) -> Result<String> {
        Ok(self
            .fetch_api::<GithubRepository>(
                format!("repos/{repo}"),
                format!("{repo}/repository.json"),
            )
            .await?
            .description
            .unwrap_or_default())
    }

    pub async fn fetch_releases(&self, repo: &str) -> Result<Vec<GithubRelease>> {
        self.fetch_api::<Vec<GithubRelease>>(
            format!("repos/{repo}/releases"),
            format!("{repo}/releases.json"),
        )
        .await
    }

    pub async fn fetch_release(&self, repo: &str, tag: &str) -> Result<GithubRelease> {
        let releases = self.fetch_releases(repo).await?;

        let release = match tag {
            "latest" => releases.first(),
            tag => releases.iter().find(|r| r.tag_name == tag),
        }
        .ok_or(anyhow!(
            "Github release '{tag}' not found on repository '{repo}'"
        ))?;

        Ok(release.clone())
    }

    pub async fn fetch_asset(
        &self,
        repo: &str,
        release_tag: &str,
        asset_name: &str,
    ) -> Result<(GithubRelease, GithubAsset)> {
        let release = self.fetch_release(repo, release_tag).await?;

        let asset = match asset_name {
            "" | "first" | "any" => release.assets.first(),
            id => {
                let id = if id.contains('$') {
                    id.replace("${tag}", release_tag)
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

    pub async fn resolve(
        &self,
        repo: &str,
        release_tag: &str,
        asset_name: &str,
    ) -> Result<(String, FileMeta)> {
        let (release, asset) = self.fetch_asset(repo, release_tag, asset_name).await?;

        let metadata = FileMeta {
            filename: asset.name.clone(),
            cache: Some(CacheLocation(
                CACHE_DIR.into(),
                format!("{repo}/releases/{}/{}", release.tag_name, asset.name),
            )),
            size: Some(asset.size),
            ..Default::default()
        };

        let url = format!(
            "https://github.com/{repo}/releases/download/{}/{}",
            release.tag_name, asset.name
        );

        Ok((url, metadata))
    }

    pub async fn resolve_steps(
        &self,
        repo: &str,
        release_tag: &str,
        asset_name: &str,
    ) -> Result<Vec<Step>> {
        let (url, metadata) = self.resolve(repo, release_tag, asset_name).await?;

        Ok(vec![
            Step::CacheCheck(metadata.clone()),
            Step::Download { url, metadata },
        ])
    }

    pub async fn resolve_remove_steps(
        &self,
        repo: &str,
        release_tag: &str,
        asset_name: &str,
    ) -> Result<Vec<Step>> {
        let (_, metadata) = self.resolve(repo, release_tag, asset_name).await?;

        Ok(vec![Step::RemoveFile(metadata)])
    }
}

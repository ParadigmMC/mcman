use std::collections::HashMap;

use anyhow::{anyhow, Context, Ok, Result};

mod models;
pub use models::*;
use serde::de::DeserializeOwned;

use crate::api::{
    app::App,
    step::{CacheLocation, FileMeta, Step},
    utils::hashing::HashFormat,
};

pub struct HangarAPI<'a>(pub &'a App);

impl<'a> HangarAPI<'a> {
    pub async fn fetch_api<T: DeserializeOwned>(&self, url: String) -> Result<T> {
        self.0
            .http_get_json(format!("{}/{url}", self.0.options.api_urls.hangar))
            .await
    }

    pub async fn fetch_project(&self, id: &str) -> Result<Project> {
        self.fetch_api(format!("projects/{id}")).await
    }

    pub async fn fetch_project_versions(&self, id: &str) -> Result<ProjectVersionsResponse> {
        self.fetch_api(format!("projects/{id}/versions")).await
    }

    pub async fn fetch_project_version(&self, id: &str, version: &str) -> Result<ProjectVersion> {
        self.fetch_api(format!("projects/{id}/versions/{version}"))
            .await
    }

    pub fn get_download_url(&self, id: &str, version: &str, platform: &str) -> String {
        format!(
            "{}/projects/{id}/versions/{version}/{platform}/download",
            self.0.options.api_urls.hangar
        )
    }

    pub async fn resolve(&self, id: &str, version_id: &str) -> Result<(String, FileMeta)> {
        let version = self
            .fetch_project_version(id, version_id)
            .await
            .context("Fetching project version")?;

        let platform = Platform::Paper; // TODO

        let download = version.downloads.get(&platform).ok_or(anyhow!(
            "Platform unsupported for Hangar project '{id}' version '{}'",
            version.name
        ))?;

        let file = download.get_file_info();

        let metadata = FileMeta {
            cache: Some(CacheLocation(
                "hangar".into(),
                format!(
                    "{}/{}/{}_{}",
                    id.split_once('/').map_or(id, |(_, id)| id),
                    version.name,
                    platform.to_string(),
                    file.name,
                ),
            )),
            filename: file.name,
            size: Some(file.size_bytes),
            hashes: HashMap::from([(HashFormat::Sha256, file.sha256_hash)]),
        };

        let url = download.get_url();

        Ok((url, metadata))
    }

    pub async fn resolve_steps(&self, id: &str, version_id: &str) -> Result<Vec<Step>> {
        let (url, metadata) = self.resolve(id, version_id).await?;

        Ok(vec![
            Step::CacheCheck(metadata.clone()),
            Step::Download { url, metadata },
        ])
    }

    pub async fn resolve_remove_steps(&self, id: &str, version_id: &str) -> Result<Vec<Step>> {
        let (_, metadata) = self.resolve(id, version_id).await?;

        Ok(vec![Step::RemoveFile(metadata)])
    }
}

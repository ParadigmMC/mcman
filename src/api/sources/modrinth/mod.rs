use anyhow::{anyhow, Result};
use models::{ModrinthFile, ModrinthProject, ModrinthVersion};
use serde::de::DeserializeOwned;

use crate::api::{app::App, step::{CacheStrategy, Step}};

mod models;
mod ratelimit;

pub struct ModrinthAPI<'a>(pub &'a App);

static API_URL: &str = "https://api.modrinth.com/v2";

impl<'a> ModrinthAPI<'a> {
    pub async fn fetch_api<T: DeserializeOwned>(&self, url: String) -> Result<T> {
        self.0.http_get_json(&*format!("{API_URL}/{url}")).await
    }

    pub async fn fetch_project(&self, id: &str) -> Result<ModrinthProject> {
        self.fetch_api(format!("project/{id}")).await
    }

    pub async fn fetch_all_versions(&self, id: &str) -> Result<Vec<ModrinthVersion>> {
        self.fetch_api(format!("project/{id}/version"))
            .await
    }

    pub async fn fetch_versions(&self, id: &str) -> Result<Vec<ModrinthVersion>> {
        Ok(self.fetch_all_versions(id).await?)
    }

    pub async fn fetch_version(&self, id: &str, version: &str) -> Result<ModrinthVersion> {
        let all_versions = self.fetch_all_versions(id).await?;

        let version_data = all_versions
            .into_iter()
            .find(|v| v.id == version || v.name == version || v.version_number == version)
            .ok_or(anyhow!(
                "Couln't find version '{version}' for Modrinth project '{id}'"
            ))?;

        Ok(version_data)
    }

    pub async fn fetch_file(
        &self,
        id: &str,
        version: &str,
    ) -> Result<(ModrinthFile, ModrinthVersion)> {
        let version = self.fetch_version(id, version).await?;

        Ok((
            version
                .files
                .iter()
                .find(|f| f.primary)
                .or(version.files.first())
                .ok_or(anyhow!(
                    "No file found on modrinth:{id}/{} ({})",
                    version.id,
                    version.name
                ))?
                .clone(),
            version,
        ))
    }

    /* pub async fn search(&self, query: &str) -> Result<Vec<ModrinthProject>> {
        Ok(self
            .0
            .http_client
            .get(format!("{API_URL}/search"))
            .query(&[("query", query), ("facets", &self.get_modrinth_facets())])
            .send()
            .await?
            .error_for_status()?
            .json::<ModrinthSearchResults>()
            .await?
            .hits)
    } */

    pub async fn version_from_hash(&self, hash: &str, algo: &str) -> Result<ModrinthVersion> {
        self.fetch_api(format!(
            "{API_URL}/version_file/{hash}{}",
            if algo.is_empty() || algo == "sha1" {
                String::new()
            } else {
                format!("?algorithm={algo}")
            }
        ))
        .await
    }

    pub async fn resolve_steps(&self, id: &str, version: &str) -> Result<Vec<Step>> {
        let (file, version) = self.fetch_file(id, version).await?;

        Ok(vec![
            Step::CacheCheck(CacheStrategy::File {
                namespace: "modrinth".into(),
                path: format!("{id}/{}/{}", version.id, file.filename),
            }),
            Step::Download {
                url: file.url,
                filename: file.filename,
                size: Some(file.size),
                hashes: file.hashes,
            },
        ])
    }
}

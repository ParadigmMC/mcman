use std::{borrow::Cow, collections::HashMap};

use anyhow::{anyhow, Context, Result};

mod models;
pub use models::*;

const HANGAR_API_URL: &str = "https://hangar.papermc.io/api/v1";

pub struct HangarAPI<'a>(pub &'a App);

impl<'a> HangarAPI<'a> {
    pub async fn fetch_api<T: DeserializeOwned>(&self, url: String) -> Result<T> {
        self.0.http_get_json(format!("{HANGAR_API_URL}/{url}")).await
    }

    pub async fn fetch_project(&self, id: &str) -> Result<Project> {
        self.fetch_api(format!("projects/{id}")).await
    }

    pub async fn fetch_project_versions(&self, id: &str, filter: Option<VersionsFilter>) -> Result<Project> {
        todo!();
    }

    pub async fn fetch_hangar_version(&self, id: &str, version: &str) -> Result<ProjectVersion> {
        let filter = self.get_versions_filter();

        let version = if version == "latest" {
            let versions =
                fetch_project_versions(&self.0.http_client, id, Some(filter))
                    .await?;

            versions
                .result
                .first()
                .ok_or(anyhow!("No compatible versions for Hangar project '{id}'"))?
                .clone()
        } else if version.contains('$') {
            let versions =
                fetch_project_versions(&self.0.http_client, id, Some(filter))
                    .await?;

            let version = version
                .replace("${mcver}", self.0.mc_version())
                .replace("${mcversion}", self.0.mc_version());

            versions
                .result
                .iter()
                .find(|v| v.name == version)
                .cloned()
                .or(versions
                    .result
                    .iter()
                    .find(|v| v.name.contains(&version))
                    .cloned())
                .ok_or(anyhow!(
                    "No compatible versions ('{version}') for Hangar project '{id}'"
                ))?
        } else {
            fetch_project_version(&self.0.http_client, id, version).await?
        };

        Ok(version)
    }

    pub fn get_platform(&self) -> Option<Platform> {
        match &self.0.server.jar {
            ServerType::Waterfall {} => Some(Platform::Waterfall),
            ServerType::Velocity {} => Some(Platform::Velocity),
            ServerType::PaperMC { project, .. } if project == "waterfall" => {
                Some(Platform::Waterfall)
            }
            ServerType::PaperMC { project, .. } if project == "velocity" => {
                Some(Platform::Velocity)
            }
            ServerType::PaperMC { project, .. } if project == "paper" => {
                Some(Platform::Paper)
            }
            ServerType::Paper {} | ServerType::Purpur { .. } => {
                Some(Platform::Paper)
            }
            _ => None,
        }
    }

    pub fn get_versions_filter(&self) -> VersionsFilter {
        let platform = self.get_platform();
        VersionsFilter {
            platform_version: if platform.is_some() {
                Some(self.0.mc_version().to_owned())
            } else {
                None
            },
            platform,
            ..Default::default()
        }
    }

    #[allow(clippy::cast_sign_loss)]
    pub async fn resolve_source(&self, id: &str, version: &str) -> Result<ResolvedFile> {
        let version = self
            .fetch_hangar_version(id, version)
            .await
            .context("Fetching project version")?;

        let download = version
            .downloads
            .get(&self.get_platform().unwrap_or(Platform::Paper))
            .ok_or(anyhow!(
                "Platform unsupported for Hangar project '{id}' version '{}'",
                version.name
            ))?;

        let cached_file_path = format!("{id}/{}/{}", version.name, download.get_file_info().name);

        Ok(ResolvedFile {
            url: download.get_url(),
            filename: download.get_file_info().name,
            cache: CacheStrategy::File {
                namespace: Cow::Borrowed("hangar"),
                path: cached_file_path,
            },
            size: Some(download.get_file_info().size_bytes as u64),
            hashes: HashMap::from([("sha256".to_owned(), download.get_file_info().sha256_hash)]),
        })
    }
}

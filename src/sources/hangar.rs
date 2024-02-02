use std::{borrow::Cow, collections::HashMap};

use anyhow::{anyhow, Context, Result};
use mcapi::hangar::{Platform, ProjectVersion};

use crate::{
    app::{App, CacheStrategy, ResolvedFile},
    model::ServerType,
};

pub struct HangarAPI<'a>(pub &'a App);

impl<'a> HangarAPI<'a> {
    pub async fn fetch_hangar_version(&self, id: &str, version: &str) -> Result<ProjectVersion> {
        let filter = self.get_versions_filter();

        let version = if version == "latest" {
            let versions =
                mcapi::hangar::fetch_project_versions(&self.0.http_client, id, Some(filter))
                    .await?;

            versions
                .result
                .first()
                .ok_or(anyhow!("No compatible versions for Hangar project '{id}'"))?
                .clone()
        } else if version.contains('$') {
            let versions =
                mcapi::hangar::fetch_project_versions(&self.0.http_client, id, Some(filter))
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
            mcapi::hangar::fetch_project_version(&self.0.http_client, id, version).await?
        };

        Ok(version)
    }

    pub fn get_platform(&self) -> Option<mcapi::hangar::Platform> {
        match &self.0.server.jar {
            ServerType::Waterfall {} => Some(mcapi::hangar::Platform::Waterfall),
            ServerType::Velocity {} => Some(mcapi::hangar::Platform::Velocity),
            ServerType::PaperMC { project, .. } if project == "waterfall" => {
                Some(mcapi::hangar::Platform::Waterfall)
            }
            ServerType::PaperMC { project, .. } if project == "velocity" => {
                Some(mcapi::hangar::Platform::Velocity)
            }
            ServerType::PaperMC { project, .. } if project == "paper" => {
                Some(mcapi::hangar::Platform::Paper)
            }
            ServerType::Paper {} | ServerType::Purpur { .. } => {
                Some(mcapi::hangar::Platform::Paper)
            }
            _ => None,
        }
    }

    pub fn get_versions_filter(&self) -> mcapi::hangar::VersionsFilter {
        let platform = self.get_platform();
        mcapi::hangar::VersionsFilter {
            platform_version: if platform.is_some() {
                Some(self.0.mc_version())
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

use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use mcapi::hangar::{Platform, ProjectVersion};

use crate::{App, CacheStrategy, ResolvedFile};

pub struct HangarAPI<'a>(pub &'a App);

impl<'a> HangarAPI<'a> {
    pub async fn fetch_hangar_version(&self, id: &str, version: &str) -> Result<ProjectVersion> {
        let filter = self
            .0
            .server
            .jar
            .get_hangar_versions_filter(&self.0.server.mc_version);

        let version = if version == "latest" {
            let versions =
                mcapi::hangar::fetch_project_versions(&self.0.http_client, id, Some(filter))
                    .await?;

            versions
                .result
                .iter()
                .next()
                .ok_or(anyhow!("No compatible versions for Hangar project '{id}'"))?
                .clone()
        } else if version.contains('$') {
            let versions =
                mcapi::hangar::fetch_project_versions(&self.0.http_client, id, Some(filter))
                    .await?;

            let version = version
                .replace("${mcver}", &self.0.mc_version())
                .replace("${mcversion}", &self.0.mc_version());

            versions
                .result
                .iter()
                .find(|v| &v.name == &version)
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

    pub async fn resolve_source(&self, id: &str, version: &str) -> Result<ResolvedFile> {
        let version = self
            .fetch_hangar_version(id, version)
            .await
            .context("Fetching project version")?;

        let download = version
            .downloads
            .get(
                &self
                    .0
                    .server
                    .jar
                    .get_hangar_platform()
                    .unwrap_or(Platform::Paper),
            )
            .ok_or(anyhow!(
                "Platform unsupported for Hangar project '{id}' version '{}'",
                version.name
            ))?;

        let cached_file_path = format!("{id}/{}/{}", version.name, download.get_file_info().name);

        Ok(ResolvedFile {
            url: download.get_url(),
            filename: download.get_file_info().name,
            cache: CacheStrategy::File {
                namespace: String::from("hangar"),
                path: cached_file_path,
            },
            size: Some(download.get_file_info().size_bytes as i32),
            hashes: HashMap::from([(
                "sha256".to_owned(),
                download.get_file_info().sha256_hash,
            )]),
        })
    }
}

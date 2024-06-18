use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;

use crate::api::{app::App, step::{CacheLocation, FileMeta, Step}};

mod models;
pub use models::*;

pub struct PaperMCAPI<'a>(pub &'a App);

const PAPERMC_URL: &str = "https://api.papermc.io/v2";
const CACHE_DIR: &str = "papermc";

impl<'a> PaperMCAPI<'a> {
    pub async fn fetch_api<T: DeserializeOwned>(
        &self,
        url: String,
    ) -> Result<T> {
        self.0.http_get_json(format!("{PAPERMC_URL}/{url}")).await
    }

    pub async fn fetch_versions(&self, project: &str) -> Result<Vec<String>> {
        let proj = self
            .fetch_api::<PaperProject>(format!("projects/{project}"))
            .await?;

        Ok(proj.versions)
    }

    pub async fn fetch_builds(&self, project: &str, version: &str) -> Result<PaperBuildsResponse> {
        let resp = self
            .fetch_api(format!(
                "projects/{project}/versions/{version}/builds"
            ))
            .await?;

        Ok(resp)
    }

    pub async fn fetch_build(
        &self,
        project: &str,
        version: &str,
        build: &str,
    ) -> Result<PaperVersionBuild> {
        let builds = self.fetch_builds(project, version).await?;
        Ok(match build {
            "latest" => builds
                .builds
                .last()
                .ok_or(anyhow!(
                    "Latest papermc build for project {project} {version} not found"
                ))?
                .clone(),
            id => builds
                .builds
                .iter()
                .find(|&b| b.build.to_string() == id)
                .ok_or(anyhow!(
                    "PaperMC build '{build}' for project {project} {version} not found"
                ))?
                .clone(),
        })
    }

    pub async fn resolve_steps(
        &self,
        project: &str,
        version: &str,
        build: &str,
    ) -> Result<Vec<Step>> {
        let resolved_build = self.fetch_build(project, &version, build).await?;

        let download = resolved_build.downloads.get("application")
            .ok_or(anyhow!("downloads['application'] missing for papermc project {project} {version}, build {build} ({})", resolved_build.build))?;

        let metadata = FileMeta {
            cache: Some(CacheLocation(CACHE_DIR.into(), format!("{project}/{}", download.name))),
            filename: download.name.clone(),
            ..Default::default()
        };

        let url = format!(
            "{PAPERMC_URL}/projects/{project}/versions/{version}/builds/{}/downloads/{}",
            resolved_build.build, download.name
        );

        Ok(vec![
            Step::CacheCheck(metadata.clone()),
            Step::Download { url, metadata },
        ])
    }
}

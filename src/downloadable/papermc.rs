use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PaperMCBuild {
    pub build: i32,
    pub channel: String,
    pub downloads: PaperMCBuildDownloads,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PaperMCBuildDownloads {
    pub application: PaperMCBuildApplication,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PaperMCBuildApplication {
    pub name: String,
    pub sha256: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PaperMCVersion {
    pub project_name: String,
    pub builds: Vec<PaperMCBuild>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PaperMCProject {
    pub project_name: String,
    pub versions: Vec<String>,
}

pub async fn fetch_papermc_versions(
    project: &str,
    client: &reqwest::Client,
) -> Result<Vec<String>> {
    let project: PaperMCProject = client
        .get("https://api.papermc.io/v2/projects/".to_owned() + project)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(project.versions)
}

pub async fn fetch_papermc_builds(
    project: &str,
    version: &str,
    client: &reqwest::Client,
) -> Result<Vec<PaperMCBuild>> {
    let mut target_version: String = version.to_owned();

    if target_version == "latest" {
        let fetched_version = fetch_papermc_versions(project, client).await?;

        if let Some(version) = fetched_version.last().cloned() {
            target_version = version;
        } else {
            bail!("Latest version for project {project} not found");
        }
    }

    let project: PaperMCVersion = client
        .get(
            "https://api.papermc.io/v2/projects/".to_owned()
                + project
                + "/versions/"
                + &target_version
                + "/builds",
        )
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(project.builds)
}

pub async fn fetch_papermc_build(
    project: &str,
    version: &str,
    build: &str,
    client: &reqwest::Client,
) -> Result<PaperMCBuild> {
    let builds = fetch_papermc_builds(project, version, client).await?;

    if build == "latest" {
        if let Some(build) = builds.last().cloned() {
            return Ok(build);
        }

        bail!("Latest build for project {project} {version} not found");
    }

    if let Some(found_build) = builds.iter().find(|&b| b.build.to_string() == build) {
        Ok(found_build.clone())
    } else {
        bail!("Build {build} for project {project} {version} not found");
    }
}

pub async fn download_papermc_build(
    project: &str,
    version: &str,
    build_id: &str,
    client: &reqwest::Client,
) -> Result<reqwest::Response> {
    let build = fetch_papermc_build(project, version, build_id, client).await?;

    let filename = build.downloads.application.name;

    let mut target_version: String = version.to_owned();

    if target_version == "latest" {
        let fetched_version = fetch_papermc_versions(project, client).await?;

        if let Some(version) = fetched_version.last().cloned() {
            target_version = version;
        } else {
            bail!("Latest version for project {project} not found");
        }
    }

    Ok(client
        .get(
            "https://api.papermc.io/v2/projects/".to_owned()
                + project
                + "/versions/"
                + &target_version
                + "/builds/"
                + &build.build.to_string()
                + "/downloads/"
                + &filename,
        )
        .send()
        .await?
        .error_for_status()?)
}

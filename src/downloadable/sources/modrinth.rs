use std::collections::HashMap;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModrinthVersion {
    pub name: String,
    pub version_number: String,
    pub changelog: String,
    pub dependencies: Vec<String>,
    pub game_versions: Vec<String>,
    pub version_type: VersionType,
    pub loaders: Vec<String>,
    pub featured: bool,
    pub status: ModrinthStatus,
    pub requested_status: Option<ModrinthStatus>,
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub date_published: String,
    pub downloads: i64,
    pub files: Vec<ModrinthFile>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModrinthDependency {
    pub version_id: String,
    pub project_id: String,
    pub file_name: String,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "snake_case")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "snake_case")]
pub enum VersionType {
    Release,
    Beta,
    Alpha,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "snake_case")]
pub enum ModrinthStatus {
    Listed,
    Archived,
    Draft,
    Unlisted,
    Scheduled,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModrinthFile {
    pub hashes: HashMap<String, String>,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: i64,
    // file_type omitted
}

pub async fn fetch_modrinth_filename(
    id: &str,
    version: &str,
    client: &reqwest::Client,
    query: Option<(&str, &str)>,
) -> Result<String> {
    let project = fetch_modrinth_versions(client, id, query).await?;

    let verdata = match version {
        "latest" => project.iter().next(),
        id => project.iter().find(|&v| v.id == id),
    };

    let Some(verdata) = verdata else {
        bail!("Release '{version}' for project '{id}' not found");
    };

    let Some(file) = verdata.files.first() else {
        bail!("No files for project '{id}' version '{version}'");
    };

    Ok(file.filename.clone())
}

pub async fn fetch_modrinth_versions(
    client: &reqwest::Client,
    id: &str,
    query: Option<(&str, &str)>,
) -> Result<Vec<ModrinthVersion>> {
    let versions: Vec<ModrinthVersion> = client
        .get("https://api.modrinth.com/v2/project/".to_owned() + id + "/version"
            + &(match query {
                Some((jar, mcver)) => format!("?loaders=[\"{jar}\"]&game_versions=[\"{mcver}\"]"),
                None => String::new(),
            }))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(versions)
}

// TODO: more complex version matching ie. mc version and server software
// TODO: also impl modrinth in mcapi and use that instead
pub async fn download_modrinth(
    id: &str,
    version: &str,
    client: &reqwest::Client,
    query: Option<(&str, &str)>,
) -> Result<reqwest::Response> {
    let project = fetch_modrinth_versions(client, id, query).await?;

    let verdata = match version {
        "latest" => project.iter().next(),
        id => project.iter().find(|&v| v.id == id),
    };

    let Some(verdata) = verdata else {
        bail!("Release '{version}' for project '{id}' not found");
    };

    let Some(file) = verdata.files.first() else {
        bail!("No files for project '{id}' version '{version}'");
    };

    let res = client.get(&file.url).send().await?.error_for_status()?;

    Ok(res)
}

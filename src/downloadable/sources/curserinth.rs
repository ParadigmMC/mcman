use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::modrinth::{ModrinthFile, ModrinthProject, VersionType};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct CurseRinthDependency {
    pub project_id: String,
    pub dependency_type: CurseRinthDependencyType,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum CurseRinthDependencyType {
    EmbeddedLibrary,
    OptionalDependency,
    RequiredDependency,
    Tool,
    Incompatible,
    Include,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CurseRinthVersion {
    pub name: String,
    pub version_number: String,
    pub changelog: String,
    pub changelog_url: String,
    pub dependencies: Vec<CurseRinthDependency>,
    pub game_versions: Vec<String>,
    pub version_type: VersionType,
    pub loaders: Vec<String>,
    pub featured: bool,
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub date_published: String,
    pub downloads: i64,
    pub files: Vec<ModrinthFile>,
}

pub static CURSERINTH_API: &str = "https://curserinth-api.kuylar.dev/v2";

pub async fn fetch_curserinth_project(
    client: &reqwest::Client,
    id: &str,
) -> Result<ModrinthProject> {
    Ok(client
        .get(CURSERINTH_API.to_owned() + "/project/" + id)
        .send()
        .await?
        .error_for_status()?
        .json::<ModrinthProject>()
        .await?)
}

pub async fn fetch_curserinth_filename(
    id: &str,
    version: &str,
    client: &reqwest::Client,
    query: Option<(&str, &str)>,
) -> Result<String> {
    let project = fetch_curserinth_versions(client, id, query).await?;

    let verdata = match version {
        "latest" => project.first(),
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

pub async fn fetch_curserinth_versions(
    client: &reqwest::Client,
    id: &str,
    query: Option<(&str, &str)>,
) -> Result<Vec<CurseRinthVersion>> {
    let versions: Vec<CurseRinthVersion> = client
        .get(
            CURSERINTH_API.to_owned()
                + "/project/"
                + id
                + "/version"
                + &(match query {
                    Some((jar, mcver)) => {
                        format!("?loaders=[\"{jar}\"]&game_versions=[\"{mcver}\"]")
                    }
                    None => String::new(),
                }),
        )
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(versions)
}

pub async fn get_curserinth_url(
    id: &str,
    version: &str,
    client: &reqwest::Client,
    query: Option<(&str, &str)>,
) -> Result<String> {
    let project = fetch_curserinth_versions(client, id, query).await?;

    let verdata = match version {
        "latest" => project.first(),
        id => project.iter().find(|&v| v.id == id),
    };

    let Some(verdata) = verdata else {
        bail!("Release '{version}' for project '{id}' not found");
    };

    let Some(file) = verdata.files.first() else {
        bail!("No files for project '{id}' version '{version}'");
    };

    Ok(file.url.clone())
}

pub async fn download_curserinth(
    id: &str,
    version: &str,
    client: &reqwest::Client,
    query: Option<(&str, &str)>,
) -> Result<reqwest::Response> {
    let url = get_curserinth_url(id, version, client, query).await?;

    let res = client.get(&url).send().await?.error_for_status()?;

    Ok(res)
}

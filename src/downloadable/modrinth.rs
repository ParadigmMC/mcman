use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ModrinthVersion {
    pub id: String,
    pub files: Vec<ModrinthFile>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ModrinthFile {
    pub url: String,
    pub filename: String,
}

pub async fn fetch_modrinth_filename(
    id: &str,
    version: &str,
    client: &reqwest::Client,
) -> Result<String> {
    let project: Vec<ModrinthVersion> = client
        .get("https://api.modrinth.com/v2/project/".to_owned() + id + "/version")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let verdata = project.iter().find(|&v| v.id == version);

    let Some(verdata) = verdata else {
        bail!("Release '{version}' for project '{id}' not found");
    };

    let Some(file) = verdata.files.first() else {
        bail!("No files for project '{id}' version '{version}'");
    };

    Ok(file.filename.clone())
}

pub async fn fetch_modrinth(
    id: &str,
    version: &str,
    client: &reqwest::Client,
) -> Result<reqwest::Response> {
    let project: Vec<ModrinthVersion> = client
        .get("https://api.modrinth.com/v2/project/".to_owned() + id + "/version")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let verdata = project.iter().find(|&v| v.id == version);

    let Some(verdata) = verdata else {
        bail!("Release '{version}' for project '{id}' not found");
    };

    let Some(file) = verdata.files.first() else {
        bail!("No files for project '{id}' version '{version}'");
    };

    let res = client.get(&file.url).send().await?.error_for_status()?;

    Ok(res)
}

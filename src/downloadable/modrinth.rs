use serde::{Deserialize, Serialize};

use crate::error::{FetchError, Result};

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

pub async fn fetch_modrinth(
    id: &str,
    version: &str,
    client: &reqwest::Client,
) -> Result<reqwest::Response> {
    let project: Vec<ModrinthVersion> = client
        .get("https://api.modrinth.com/v2/project/".to_owned() + id + "/version")
        .send()
        .await?
        .json()
        .await?;

    let verdata = project.iter().find(|&v| v.id == version);

    if verdata.is_none() {
        Err(FetchError::ModrinthReleaseNotFound(
            id.to_owned(),
            version.to_owned(),
        ))?;
    }

    let file = verdata.unwrap().files.first();

    if file.is_none() {
        Err(FetchError::ModrinthReleaseNotFound(
            id.to_owned(),
            version.to_owned(),
        ))?;
    }

    let res = client.get(&file.unwrap().url).send().await?;

    Ok(res)
}

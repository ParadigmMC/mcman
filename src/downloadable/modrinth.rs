use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::error::{Result, Error};

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
) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>> {
    let project: Vec<ModrinthVersion> = client
        .get("https://api.modrinth.com/v2/project/".to_owned() + id + "/version")
        .send()
        .await?
        .json()
        .await?;

    let verdata = project
        .iter()
        .find(|&v| v.id == version);

    if verdata.is_none() {
        return Err(Error::ModrinthReleaseNotFound(id.to_owned(), version.to_owned()));
    }

    let file = verdata
        .unwrap()
        .files.first();

    if file.is_none() {
        return Err(Error::ModrinthReleaseNotFound(id.to_owned(), version.to_owned()));
    }

    let req = client
        .get(&file.unwrap().url)
        .send()
        .await?
        .bytes_stream();

    Ok(req)
}

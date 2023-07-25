use anyhow::{bail, Result};

use super::modrinth::{ModrinthProject, ModrinthVersion};

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
) -> Result<Vec<ModrinthVersion>> {
    let versions: Vec<ModrinthVersion> = client
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

pub async fn download_curserinth(
    id: &str,
    version: &str,
    client: &reqwest::Client,
    query: Option<(&str, &str)>,
) -> Result<reqwest::Response> {
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

    let res = client.get(&file.url).send().await?.error_for_status()?;

    Ok(res)
}

use anyhow::{anyhow, Result};

pub async fn fetch_vanilla(version: &str, client: &reqwest::Client) -> Result<reqwest::Response> {
    let version_manifest = mcapi::vanilla::fetch_version_manifest(client).await?;

    Ok(match version {
        "latest" => version_manifest.fetch_latest_release(client).await?,
        "latest-snapshot" => version_manifest.fetch_latest_snapshot(client).await?,
        id => version_manifest.fetch(id, client).await?,
    }
    .downloads
    .get(&mcapi::vanilla::DownloadType::Server)
    .ok_or(anyhow!(
        "version manifest doesn't include a server download"
    ))?
    .download(client)
    .await?)
}

pub async fn fetch_latest_mcver(client: &reqwest::Client) -> Result<String> {
    let version_manifest = mcapi::vanilla::fetch_version_manifest(client).await?;

    Ok(version_manifest.latest.release)
}

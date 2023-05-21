use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct PurpurMCResponse {
    pub project: String,
    pub versions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PurpurMCVersion {
    pub builds: PurpurMCBuilds,
    pub project: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PurpurMCBuilds {
    pub latest: String,
    pub all: Vec<String>,
}

pub async fn fetch_purpurmc_versions(client: &reqwest::Client) -> Result<Vec<String>> {
    let project: PurpurMCResponse = client
        .get("https://api.purpurmc.org/v2/purpur".to_owned())
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(project.versions)
}

pub async fn fetch_purpurmc_builds(version: &str, client: &reqwest::Client) -> Result<Vec<String>> {
    let mut target_version: String = version.to_owned();

    if target_version == "latest" {
        let fetched_version = fetch_purpurmc_versions(client).await?;

        if let Some(version) = fetched_version.last().cloned() {
            target_version = version;
        } else {
            bail!("Latest version for Purpur not found");
        }
    }

    let version: PurpurMCVersion = client
        .get("https://api.purpurmc.org/v2/purpur/".to_owned() + &target_version)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(version.builds.all)
}

pub async fn download_purpurmc_build(
    version: &str,
    build: &str,
    client: &reqwest::Client,
) -> Result<reqwest::Response> {
    Ok(client
        .get("https://api.purpurmc.org/v2/purpur/".to_owned() + version + "/" + build + "/download")
        .send()
        .await?
        .error_for_status()?)
}

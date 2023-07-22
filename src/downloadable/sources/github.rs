use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::util::match_artifact_name;

async fn wait_ratelimit(res: reqwest::Response) -> Result<reqwest::Response> {
    let res = if let Some(h) = res.headers().get("x-ratelimit-remaining") {
        if String::from_utf8_lossy(h.as_bytes()) == "1" {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            let ratelimit_reset =
                String::from_utf8_lossy(res.headers()["x-ratelimit-reset"].as_bytes())
                    .parse::<u64>()?;
            let amount = ratelimit_reset - now;
            println!(" (!) Github ratelimit exceeded. sleeping for {amount} seconds...");
            sleep(Duration::from_secs(amount)).await;
        }
        res
    } else {
        res.error_for_status()?
    };

    Ok(res)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubRelease {
    pub tag_name: String,
    pub name: String,
    pub assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubAsset {
    pub url: String,
    pub name: String,
}

pub async fn fetch_github_releases(
    repo: &str,
    client: &reqwest::Client,
) -> Result<Vec<GithubRelease>> {
    let releases: Vec<GithubRelease> = wait_ratelimit(
        client
            .get("https://api.github.com/repos/".to_owned() + repo + "/releases")
            .send()
            .await?,
    )
    .await?
    .json()
    .await?;

    Ok(releases)
}

pub async fn fetch_github_release_asset(
    repo: &str,
    tag: &str,
    asset: &str,
    client: &reqwest::Client,
) -> Result<GithubAsset> {
    let releases = fetch_github_releases(repo, client).await?;

    let release = match tag {
        "latest" => releases.into_iter().next(),
        id => releases.into_iter().find(|r| r.tag_name == id),
    }
    .ok_or(anyhow!("release not found"))?;

    let resolved_asset = match asset {
        "" | "first" | "any" => release.assets.into_iter().next(),
        id => release
            .assets
            .into_iter()
            .find(|a| match_artifact_name(id, &a.name)),
    }
    .ok_or(anyhow!("asset not found"))?;

    Ok(resolved_asset)
}

pub async fn fetch_github_release_filename(
    repo: &str,
    tag: &str,
    asset: &str,
    client: &reqwest::Client,
) -> Result<String> {
    Ok(fetch_github_release_asset(repo, tag, asset, client)
        .await?
        .name)
}

pub async fn download_github_release(
    repo: &str,
    tag: &str,
    asset: &str,
    client: &reqwest::Client,
    filename_hint: Option<&str>,
) -> Result<reqwest::Response> {
    let filename = if let Some(filename) = filename_hint {
        filename.to_owned()
    } else {
        let fetched_asset = fetch_github_release_asset(repo, tag, asset, client).await?;
        fetched_asset.name
    };

    Ok(wait_ratelimit(
        client
            .get(format!(
                "https://github.com/{repo}/releases/download/{tag}/{}",
                filename
            ))
            .send()
            .await?,
    )
    .await?
    .error_for_status()?)
}

pub async fn fetch_repo_description(client: &reqwest::Client, repo: &str) -> Result<String> {
    let desc = wait_ratelimit(
        client
            .get("https://api.github.com/repos/".to_owned() + repo)
            .send()
            .await?,
    )
    .await?
    .error_for_status()?
    .json::<serde_json::Value>()
    .await?["description"]
        .as_str()
        .unwrap_or_default()
        .to_owned();

    Ok(desc)
}

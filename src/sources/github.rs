use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubRelease {
    pub tag_name: String,
    pub name: String,
    pub assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    mcver: &str,
    client: &reqwest::Client,
) -> Result<GithubAsset> {
    let releases = fetch_github_releases(repo, client).await?;

    let tag = tag.replace("${mcver}", mcver);
    let tag = tag.replace("${mcversion}", mcver);

    let release = match tag.as_str() {
        "latest" => releases.first(),
        id => releases.iter().find(|r| r.tag_name == id),
    }
    .ok_or(anyhow!(
        "Github release with tag '{tag}' not found on repository '{repo}'"
    ))?;

    let assets = &release.assets;

    let resolved_asset = match asset {
        "" | "first" | "any" => assets.first(),
        id => {
            let id = if id.contains('$') {
                id.replace("${version}", &release.tag_name)
                    .replace("${tag}", &release.tag_name)
                    .replace("${release}", &release.tag_name)
                    .replace("${mcver}", mcver)
                    .replace("${mcversion}", mcver)
            } else {
                id.to_owned()
            };

            assets
                .iter()
                .find(|a| id == a.name)
                .or(assets.iter().find(|a| a.name.contains(&id)))
        }
    }
    .ok_or(anyhow!(
        "Github release asset with name '{asset}' on release '{}' not found",
        release.tag_name
    ))?;

    Ok(resolved_asset.clone())
}

pub async fn fetch_github_release_filename(
    repo: &str,
    tag: &str,
    asset: &str,
    mcver: &str,
    client: &reqwest::Client,
) -> Result<String> {
    Ok(fetch_github_release_asset(repo, tag, asset, mcver, client)
        .await?
        .name)
}

pub async fn get_github_release_url(
    repo: &str,
    tag: &str,
    asset: &str,
    mcver: &str,
    client: &reqwest::Client,
    filename_hint: Option<&str>,
) -> Result<String> {
    let filename = if let Some(filename) = filename_hint {
        filename.to_owned()
    } else {
        let fetched_asset = fetch_github_release_asset(repo, tag, asset, mcver, client).await?;
        fetched_asset.name
    };

    Ok(format!(
        "https://github.com/{repo}/releases/download/{tag}/{filename}"
    ))
}

pub async fn download_github_release(
    repo: &str,
    tag: &str,
    asset: &str,
    mcver: &str,
    client: &reqwest::Client,
    filename_hint: Option<&str>,
) -> Result<reqwest::Response> {
    let filename = if let Some(filename) = filename_hint {
        filename.to_owned()
    } else {
        let fetched_asset = fetch_github_release_asset(repo, tag, asset, mcver, client).await?;
        fetched_asset.name
    };

    Ok(wait_ratelimit(
        client
            .get(format!(
                "https://github.com/{repo}/releases/download/{tag}/{filename}"
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

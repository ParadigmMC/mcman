#![allow(dead_code)] // todo...
#![allow(unused)]

use anyhow::{anyhow, Result};
use mcapi::quilt::{InstallerVariant, self};

pub async fn download_quilt_installer(
    client: &reqwest::Client,
    installer: &str,
) -> Result<reqwest::Response> {
    let v = match installer {
        "latest" => fetch_latest_quilt_installer(client).await?,
        id => id.to_owned(),
    };

    Ok(quilt::download_installer(client, &InstallerVariant::Universal, &v).await?)
}

pub async fn fetch_latest_quilt_installer(
    client: &reqwest::Client
) -> Result<String> {
    Ok(
        mcapi::quilt::fetch_installer_versions(client, &InstallerVariant::Universal).await?
            .last().expect("latest quilt installer version to be present").clone()
    )
}

pub async fn get_installer_filename(
    client: &reqwest::Client,
    installer: &str,
) -> Result<String> {
    let v = match installer {
        "latest" => fetch_latest_quilt_installer(client).await?,
        id => id.to_owned(),
    };

    Ok(format!("quilt-installer-{v}.jar"))
}


/* 

pub async fn fetch_quilt_latest_loader(client: &reqwest::Client) -> Result<String> {
    let loaders = mcapi::quilt::fetch_loaders(client).await?;

    Ok(loaders
        .first()
        .ok_or(anyhow!("Couldn't get latest quilt loader"))?
        .version
        .clone())
}

pub async fn fetch_quilt_latest_installer(client: &reqwest::Client) -> Result<String> {
    let installers = mcapi::quilt::fetch_installers(client).await?;

    Ok(installers
        .first()
        .ok_or(anyhow!("Couldn't get latest quilt installer"))?
        .version
        .clone())
}

pub async fn download_quilt_installer(
    client: &reqwest::Client,
    mcver: &str,
    loader: &str,
    installer: &str,
) -> Result<reqwest::Response> {
    Ok(mcapi::quilt::download_installer_jar(
        client,
        &match installer {
            "latest" => fetch_quilt_latest_installer(client).await?,
            id => id.to_owned(),
        },
    )
    .await?)
}

pub static QUILT_DEFAULT_SERVERJAR_FILENAME: &str = "quilt-server-launch.jar";

pub async fn get_quilt_filename(
    client: &reqwest::Client,
    mcver: &str,
    loader: &str,
) -> Result<String> {
    let l = match loader {
        "latest" => fetch_quilt_latest_loader(client).await?,
        id => id.to_owned(),
    };

    Ok(format!("quilt-server-{mcver}-{l}-launch.jar"))
}
 */
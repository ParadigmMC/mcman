use anyhow::{anyhow, Result};

pub async fn fetch_fabric_latest_loader(client: &reqwest::Client) -> Result<String> {
    let loaders = mcapi::fabric::fetch_loaders(client).await?;

    Ok(loaders
        .first()
        .ok_or(anyhow!("Couldn't get latest fabric loader"))?
        .version
        .clone())
}

pub async fn fetch_fabric_latest_installer(client: &reqwest::Client) -> Result<String> {
    let installers = mcapi::fabric::fetch_installers(client).await?;

    Ok(installers
        .first()
        .ok_or(anyhow!("Couldn't get latest fabric installer"))?
        .version
        .clone())
}

pub async fn download_fabric(
    client: &reqwest::Client,
    mcver: &str,
    loader: &str,
    installer: &str,
) -> Result<reqwest::Response> {
    Ok(mcapi::fabric::download_server_jar(
        client,
        mcver,
        &match loader {
            "latest" => fetch_fabric_latest_loader(client).await?,
            id => id.to_owned(),
        },
        &match installer {
            "latest" => fetch_fabric_latest_installer(client).await?,
            id => id.to_owned(),
        },
    )
    .await?)
}

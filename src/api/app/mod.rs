use std::{sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};

use anyhow::{Context, Result};
use reqwest::{IntoUrl, Url};
use serde::de::DeserializeOwned;
use tokio::{sync::RwLock, time::sleep};

use super::models::{network::Network, Addon, server::Server};

pub mod actions;

pub const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " - ",
    env!("CARGO_PKG_REPOSITORY"),
);

pub struct App {
    pub http_client: reqwest::Client,
    pub server: Option<Arc<RwLock<Server>>>,
    pub network: Option<Arc<RwLock<Network>>>,
    pub ci: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .context("Initializing http_client")?;

        Ok(Self {
            http_client,
            server: None,
            network: None,
            ci: false,
        })
    }

    pub async fn http_get_json<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T> {
        let res = self.http_client
            .get(url.as_str())
            .send()
            .await?
            .error_for_status()?;

        if res.headers().get("x-ratelimit-remaining").map(|x| String::from_utf8_lossy(x.as_bytes())) == Some("1".into()) {
            let ratelimit_reset = String::from_utf8_lossy(res.headers()["x-ratelimit-reset"].as_bytes())
                .parse::<u64>()?;
            let sleep_amount = match url.into_url()?.domain() {
                Some("github.com") => {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                    let amount = ratelimit_reset - now;
                    Some(amount)
                },
                Some("modrinth.com") => {
                    Some(ratelimit_reset)
                },
                _ => None,
            };
            
            if let Some(amount) = sleep_amount {
                sleep(Duration::from_secs(amount)).await;
            }
        }
            
        Ok(res.json().await?)
    }

    pub async fn collect_addons(&self) -> Result<Vec<Addon>> {
        let mut addons = vec![];

        if let Some(lock) = &self.server {
            let server = lock.read().await;

            for source in &server.sources {
                addons.append(&mut source.resolve(&self).await?);
            }
        }

        Ok(addons)
    }
}

use std::sync::Arc;

use anyhow::{Context, Result};
use reqwest::Url;
use serde::de::DeserializeOwned;
use tokio::sync::RwLock;

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

    pub async fn http_get_json<T: DeserializeOwned>(&self, url: impl Into<Url>) -> Result<T> {
        Ok(self.http_client
            .get(url.into())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?
        )
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

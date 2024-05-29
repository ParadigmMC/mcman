use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::sync::RwLock;

use super::models::{network::Network, Addon, Server};

pub mod actions;

pub const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " - ",
    env!("CARGO_PKG_REPOSITORY"),
);

pub struct App {
    http_client: reqwest::Client,
    server: Option<Arc<RwLock<Server>>>,
    network: Option<Arc<RwLock<Network>>>,
    ci: bool,
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

    pub async fn collect_addons(&self) -> Result<Vec<Addon>> {
        let mut addons = vec![];

        if let Some(lock) = &self.server {
            let server = lock.read().await;

            for source in &server.sources {
                addons.append(&mut source.resolve().await?);
            }
        }

        Ok(addons)
    }
}

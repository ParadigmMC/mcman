use std::{
    path::PathBuf, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}
};

use anyhow::{Context, Result};
use cache::Cache;
use reqwest::{IntoUrl, Response};
use serde::de::DeserializeOwned;
use tokio::{sync::RwLock, time::sleep};

use super::{models::{network::{Network, NETWORK_TOML}, server::{Server, SERVER_TOML}, Addon}, utils::{try_find_toml_upwards, write_toml}};

pub mod actions;
pub mod cache;
pub mod options;

pub const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " - ",
    env!("CARGO_PKG_REPOSITORY"),
);

pub struct App {
    pub http_client: reqwest::Client,
    pub server_path: Option<PathBuf>,
    pub server: Option<Arc<RwLock<Server>>>,
    pub network_path: Option<PathBuf>,
    pub network: Option<Arc<RwLock<Network>>>,
    pub cache: Cache,
    pub ci: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .context("Initializing http_client")?;

        let cache = Cache::new(dirs::cache_dir());

        let (server_path, server) = try_find_toml_upwards::<Server>(SERVER_TOML)?.unzip();
        let (network_path, network) = try_find_toml_upwards::<Network>(NETWORK_TOML)?.unzip();

        Ok(Self {
            http_client,
            server_path,
            server: server.map(|s| Arc::new(RwLock::new(s))),
            network_path,
            network: network.map(|nw| Arc::new(RwLock::new(nw))),
            cache,
            ci: false,
        })
    }

    pub async fn save_changes(&self) -> Result<()> {
        if let Some((path, server)) = self.server_path.as_ref().zip(self.server.as_ref()) {
            let server = server.read().await;
            write_toml(&path, SERVER_TOML, &*server)?;
        }

        if let Some((path, network)) = self.network_path.as_ref().zip(self.network.as_ref()) {
            let network = network.read().await;
            write_toml(&path, NETWORK_TOML, &*network)?;
        }

        Ok(())
    }

    pub async fn http_get(&self, url: impl IntoUrl) -> Result<Response> {
        let res = self
            .http_client
            .get(url.as_str())
            .send()
            .await?
            .error_for_status()?;

        if res
            .headers()
            .get("x-ratelimit-remaining")
            .map(|x| String::from_utf8_lossy(x.as_bytes()))
            == Some("1".into())
        {
            let ratelimit_reset =
                String::from_utf8_lossy(res.headers()["x-ratelimit-reset"].as_bytes())
                    .parse::<u64>()?;
            let sleep_amount = match url.into_url()?.domain() {
                Some("github.com") => {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                    let amount = ratelimit_reset - now;
                    Some(amount)
                }
                Some("modrinth.com") => Some(ratelimit_reset),
                _ => None,
            };

            if let Some(amount) = sleep_amount {
                sleep(Duration::from_secs(amount)).await;
            }
        }

        Ok(res)
    }

    pub async fn http_get_json<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T> {
        let res = self.http_get(url).await?;
        Ok(res.json().await?)
    }

    pub async fn collect_addons(&self) -> Result<Vec<Addon>> {
        let mut addons = vec![];

        if let Some(lock) = &self.server {
            let server = lock.read().await;

            for source in &server.sources {
                addons.append(&mut source.resolve_addons(&self).await?);
            }
        }

        Ok(addons)
    }
}

macro_rules! api_methods {
    ($(
        $name:ident => $t:ident,
    )*) => {$(
        pub fn $name(&self) -> crate::api::sources::$name::$t<'_> {
            crate::api::sources::$name::$t(&self)
        }
    )*};
}

impl App {
    api_methods! {
        vanilla => VanillaAPI,
        modrinth => ModrinthAPI,
    }
}

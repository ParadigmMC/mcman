use std::{path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use cache::Cache;
use confique::Config;
use options::AppOptions;
use tokio::sync::RwLock;

use super::models::{network::Network, server::Server};

pub mod actions;
pub mod cache;
mod collect;
mod http;
mod io;
pub mod options;
mod step;

pub const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " - https://mcman.deniz.blue",
);

pub struct App {
    pub http_client: reqwest::Client,
    pub server_path: Option<PathBuf>,
    pub server: Option<Arc<RwLock<Server>>>,
    pub network_path: Option<PathBuf>,
    pub network: Option<Arc<RwLock<Network>>>,
    pub cache: Cache,
    pub options: AppOptions,
    pub ci: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .context("Initializing http_client")?;

        let ci = std::env::var("CI").is_ok_and(|v| v == "true");

        let options = AppOptions::builder()
            .env()
            .file(".mcman.toml")
            .file(
                dirs::config_dir()
                    .unwrap_or_default()
                    .join("mcman/.mcman.toml"),
            )
            .load()?;

        let cache = Cache::new(if options.disable_cache {
            None
        } else {
            dirs::cache_dir().map(|p| p.join("mcman"))
        });

        let mut app = Self {
            http_client,
            server_path: None,
            server: None,
            network_path: None,
            network: None,
            cache,
            options,
            ci,
        };

        app.try_read_files()?;

        Ok(app)
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
        github => GithubAPI,
        papermc => PaperMCAPI,
        fabric => FabricAPI,
        maven => MavenAPI,
    }
}

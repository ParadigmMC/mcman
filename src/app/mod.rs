mod hashing;
mod downloading;
mod progress;
mod from_string;
mod caching;
mod resolvable;

use anyhow::{Result, Context};
use indicatif::MultiProgress;
pub use resolvable::*;
pub use caching::*;

use crate::sources;
use crate::model::{Network, Server};

pub const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " - ",
    env!("CARGO_PKG_REPOSITORY"),
);

pub struct BaseApp {
    pub http_client: reqwest::Client,
}

impl BaseApp {
    pub fn new() -> Result<Self> {
        let b = reqwest::Client::builder().user_agent(APP_USER_AGENT);

        Ok(Self {
            http_client: b.build().context("Failed to create HTTP client")?
        })
    }

    pub fn upgrade(self) -> Result<App> {
        Ok(App {
            http_client: self.http_client,
            server: Server::load().context("Failed to load server.toml")?,
            network: Network::load()?,
            multi_progress: MultiProgress::new(),
        })
    }

    pub fn upgrade_with_default_server(self) -> Result<App> {
        Ok(App {
            http_client: self.http_client,
            server: Server::default(),
            network: Network::load()?,
            multi_progress: MultiProgress::new(),
        })
    }
}

#[derive(Debug)]
pub struct App {
    pub http_client: reqwest::Client,
    pub server: Server,
    pub network: Option<Network>,

    pub multi_progress: MultiProgress,
}

impl App {
    pub fn new() -> Result<Self> {
        BaseApp::new()?.upgrade()
    }

    pub fn mc_version(&self) -> String {
        self.server.mc_version.clone()
    }

    pub fn get_cache(&self, ns: &str) -> Option<Cache> {
        // TODO check if cache should be enabled to return None
        Cache::get_cache(ns)
    }

    pub fn has_in_cache(&self, ns: &str, path: &str) -> bool {
        self.get_cache(ns)
            .map(|c| c.exists(path))
            .unwrap_or(false)
    }
}

impl<'a> App {
    pub fn github(&'a self) -> sources::github::GithubAPI<'a> {
        sources::github::GithubAPI(&self)
    }

    pub fn modrinth(&'a self) -> sources::modrinth::ModrinthAPI<'a> {
        sources::modrinth::ModrinthAPI(&self)
    }

    pub fn curserinth(&'a self) -> sources::curserinth::CurserinthAPI<'a> {
        sources::curserinth::CurserinthAPI(&self)
    }

    pub fn hangar(&'a self) -> sources::hangar::HangarAPI<'a> {
        sources::hangar::HangarAPI(&self)
    }

    pub fn fabric(&'a self) -> sources::fabric::FabricAPI<'a> {
        sources::fabric::FabricAPI(&self)
    }

    pub fn maven(&'a self) -> sources::maven::MavenAPI<'a> {
        sources::maven::MavenAPI(&self)
    }

    pub fn quilt(&'a self) -> sources::quilt::QuiltAPI<'a> {
        sources::quilt::QuiltAPI(&self)
    }

    pub fn forge(&'a self) -> sources::forge::ForgeAPI<'a> {
        sources::forge::ForgeAPI(&self)
    }

    pub fn neoforge(&'a self) -> sources::neoforge::NeoforgeAPI<'a> {
        sources::neoforge::NeoforgeAPI(&self)
    }

    pub fn papermc(&'a self) -> sources::papermc::PaperMCAPI<'a> {
        sources::papermc::PaperMCAPI(&self)
    }

    pub fn purpurmc(&'a self) -> sources::purpur::PurpurAPI<'a> {
        sources::purpur::PurpurAPI(&self)
    }

    pub fn spigot(&'a self) -> sources::spigot::SpigotAPI<'a> {
        sources::spigot::SpigotAPI(&self)
    }

    pub fn vanilla(&'a self) -> sources::vanilla::VanillaAPI<'a> {
        sources::vanilla::VanillaAPI(&self)
    }
}

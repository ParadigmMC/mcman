mod hashing;
mod downloading;
mod progress;
mod from_string;
mod caching;
mod resolvable;
mod actions;

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

macro_rules! api_methods {
    ($name:ident, $t:ident) => {
        pub fn $name(&'a self) -> sources::$name::$t<'a> {
            sources::$name::$t(&self)
        }
    };
}

impl<'a> App {
    api_methods!(vanilla, VanillaAPI);

    api_methods!(github, GithubAPI);
    api_methods!(maven, MavenAPI);
    
    api_methods!(modrinth, ModrinthAPI);
    api_methods!(curserinth, CurserinthAPI);

    api_methods!(neoforge, NeoforgeAPI);
    api_methods!(forge, ForgeAPI);
    api_methods!(fabric, FabricAPI);
    api_methods!(quilt, QuiltAPI);
    
    api_methods!(papermc, PaperMCAPI);
    api_methods!(hangar, HangarAPI);
    api_methods!(purpur, PurpurAPI);
    api_methods!(spigot, SpigotAPI);

    pub fn markdown(&'a self) -> crate::interop::markdown::MarkdownAPI<'a> {
        crate::interop::markdown::MarkdownAPI(&self)
    }

    pub fn packwiz(mut self) -> crate::interop::packwiz::PackwizInterop<'a> {
        crate::interop::packwiz::PackwizInterop(&mut self)
    }

    pub fn mrpack(mut self) -> crate::interop::mrpack::MRPackInterop<'a> {
        crate::interop::mrpack::MRPackInterop(&mut self)
    }
}

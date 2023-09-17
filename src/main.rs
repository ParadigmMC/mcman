#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]
#![allow(unknown_lints)]

use std::{path::PathBuf, collections::HashMap};

use anyhow::{Context, Result};
use async_trait::async_trait;
use cache::Cache;
use clap::Parser;
use model::{Server, Network};

mod commands;
mod core;
mod model;
mod sources;
mod util;
mod cache;
//mod hot_reload;

#[derive(clap::Parser)]
#[command(author = "ParadigmMC", color = clap::ColorChoice::Always)]
#[command(about = "Powerful Minecraft Server Manager CLI")]
#[command(after_help = "To start building servers, try 'mcman init'")]
#[command(subcommand_required = true, arg_required_else_help = true)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Initialize a new mcman server
    Init(commands::init::Args),
    /// Build using server.toml configuration
    Build(commands::build::Args),
    /// Test the server (stops it when it ends startup)
    Run(commands::run::Args),
    /// Add a plugin/mod/datapack
    #[command(subcommand)]
    Add(commands::add::Commands),
    /// Importing tools
    #[command(subcommand)]
    Import(commands::import::Commands),
    /// Update markdown files with server info
    #[command(visible_alias = "md")]
    Markdown,
    /// Pull files from server/ to config/
    Pull(commands::pull::Args),
    /// Helpers for setting up the environment
    #[command(subcommand)]
    Env(commands::env::Commands),
    /// Pack or unpack a world
    #[command(subcommand, visible_alias = "w")]
    World(commands::world::Commands),
    /// Show info about the server in console
    Info,
    /// Show version information
    #[command(visible_alias = "v")]
    Version,
    /// Exporting tools
    #[command(subcommand)]
    Export(commands::export::Commands),
    /// Eject - remove everything related to mcman
    #[command(hide = true)]
    Eject,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CLI::parse();

    match args.command {
        Commands::Init(args) => commands::init::run(args).await,
        Commands::Build(args) => commands::build::run(args).await.map(|_| ()),
        Commands::Run(args) => commands::run::run(args).await,
        Commands::Add(commands) => commands::add::run(commands).await,
        Commands::Import(subcommands) => commands::import::run(subcommands).await,
        Commands::Markdown => commands::markdown::run().await,
        Commands::Pull(args) => commands::pull::run(args),
        Commands::Env(commands) => commands::env::run(commands),
        Commands::World(commands) => commands::world::run(commands).await,
        Commands::Info => commands::info::run(),
        Commands::Version => commands::version::run().await,
        Commands::Export(commands) => commands::export::run(commands).await,
        Commands::Eject => commands::eject::run(),
    }
}

pub const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " - ",
    env!("CARGO_PKG_REPOSITORY"),
);

pub fn create_http_client() -> Result<reqwest::Client> {
    let b = reqwest::Client::builder().user_agent(APP_USER_AGENT);

    b.build().context("Failed to create HTTP client")
}

#[async_trait]
pub trait Source {
    async fn resolve_source(
        &self,
        app: &App,
    ) -> Result<FileSource>;
}

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

    fn into_app(self) -> Result<App> {
        Ok(App {
            http_client: self.http_client,
            server: Server::load().context("Failed to load server.toml")?,
            network: Network::load()?
        })
    }
}

pub struct App {
    pub http_client: reqwest::Client,
    pub server: Server,
    pub network: Option<Network>,
}

impl App {
    pub fn new() -> Result<Self> {
        BaseApp::new()?.into_app()
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

pub enum FileSource {
    Download {
        url: String,
        filename: String,
        cache: CacheStrategy,
        size: Option<i32>,
        hashes: HashMap<String, String>,
    },

    Cached {
        path: PathBuf,
        filename: String,
    }
}

pub enum CacheStrategy {
    File {
        path: PathBuf,
    },
    Indexed {
        index_path: PathBuf,
        key: String,
        value: PathBuf,
    },
    None,
}

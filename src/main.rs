#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]
#![allow(unknown_lints)]
// its used mutably, silly rustc
#![allow(unused_mut)]

use anyhow::Result;
use app::BaseApp;
use clap::Parser;

mod commands;
mod core;
mod app;
mod model;
mod sources;
mod util;
mod interop;
mod hot_reload;

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
    Build(commands::build::BuildArgs),
    /// Test the server (stops it when it ends startup)
    Run(commands::run::RunArgs),
    /// Add a plugin/mod/datapack
    #[command(subcommand)]
    Add(commands::add::Commands),
    /// Importing tools
    #[command(subcommand)]
    Import(commands::import::Commands),
    /// Update markdown files with server info
    #[command(visible_alias = "md")]
    Markdown,
    /// Download a downloadable
    #[command(visible_alias = "dl")]
    Download(commands::download::Args),
    /// Cache management commands
    #[command(subcommand)]
    Cache(commands::cache::Commands),
    /// Start a dev session
    Dev,
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

    let mut base_app = BaseApp::new()?;

    match args.command {
        Commands::Init(args) => commands::init::run(base_app, args).await,
        Commands::Build(args) => commands::build::run(base_app.upgrade()?, args).await.map(|_| ()),
        Commands::Run(args) => commands::run::run(base_app.upgrade()?, args).await,
        Commands::Add(commands) => commands::add::run(base_app.upgrade()?, commands).await,
        Commands::Import(subcommands) => commands::import::run(base_app.upgrade()?, subcommands).await,
        Commands::Markdown => commands::markdown::run(base_app.upgrade()?, ).await,
        Commands::Download(args) => commands::download::run(base_app.upgrade()?, args).await,
        Commands::Cache(subcommands) => commands::cache::run(subcommands).await,
        Commands::Dev => commands::dev::run(base_app.upgrade()?).await,
        Commands::Pull(args) => commands::pull::run(base_app.upgrade()?, args),
        Commands::Env(commands) => commands::env::run(base_app.upgrade()?, commands),
        Commands::World(commands) => commands::world::run(base_app.upgrade()?, commands).await,
        Commands::Info => commands::info::run(base_app.upgrade()?),
        Commands::Version => commands::version::run(base_app).await,
        Commands::Export(commands) => commands::export::run(base_app.upgrade()?, commands).await,
        Commands::Eject => commands::eject::run(base_app.upgrade()?),
    }
}

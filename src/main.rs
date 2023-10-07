#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]
#![allow(unknown_lints)]

use std::collections::HashMap;

use anyhow::{Context, Result};
use async_trait::async_trait;
use cache::Cache;
use clap::Parser;
use indicatif::MultiProgress;
use model::{Server, Network};
use serde::{Serialize, Deserialize};

mod commands;
mod core;
mod app;
mod model;
mod sources;
mod util;
mod cache;
mod interop;
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

    let base_app = BaseApp::new()?;

    match args.command {
        Commands::Init(args) => commands::init::run(base_app, args).await,
        Commands::Build(args) => commands::build::run(base_app.upgrade(), args).await.map(|_| ()),
        Commands::Run(args) => commands::run::run(base_app.upgrade(), args).await,
        Commands::Add(commands) => commands::add::run(base_app.upgrade(), commands).await,
        Commands::Import(subcommands) => commands::import::run(base_app.upgrade(), subcommands).await,
        Commands::Markdown => commands::markdown::run(base_app.upgrade(), ).await,
        Commands::Pull(args) => commands::pull::run(base_app.upgrade(), args),
        Commands::Env(commands) => commands::env::run(base_app.upgrade(), commands),
        Commands::World(commands) => commands::world::run(base_app.upgrade(), commands).await,
        Commands::Info => commands::info::run(base_app.upgrade()),
        Commands::Version => commands::version::run(base_app).await,
        Commands::Export(commands) => commands::export::run(base_app.upgrade(), commands).await,
        Commands::Eject => commands::eject::run(base_app.upgrade(), ),
    }
}

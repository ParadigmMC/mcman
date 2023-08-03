#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]
#![allow(unknown_lints)]

use anyhow::{Context, Result};
use async_trait::async_trait;
use clap::Command;

mod bootstrapper;
mod commands;
mod core;
mod model;
mod sources;
mod util;
//mod hot_reload;

fn cli() -> Command {
    Command::new("mcman")
        .about("Powerful Minecraft Server Manager CLI")
        .after_help("To start building servers, try 'mcman init'")
        .author("ParadigmMC")
        .color(clap::ColorChoice::Always)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(commands::init::cli())
        .subcommand(commands::build::cli())
        .subcommand(commands::run::cli())
        .subcommand(commands::import::cli())
        .subcommand(commands::markdown::cli())
        .subcommand(commands::pull::cli())
        .subcommand(commands::env::cli())
        .subcommand(commands::info::cli())
        .subcommand(commands::version::cli())
        .subcommand(commands::export::cli())
        .subcommand(commands::eject::cli())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => commands::init::run(sub_matches).await,
        Some(("build", sub_matches)) => commands::build::run(sub_matches).await.map(|_| ()),
        Some(("run", sub_matches)) => commands::run::run(sub_matches).await,
        Some(("import" | "i", sub_matches)) => commands::import::run(sub_matches).await,
        Some(("markdown" | "md", _)) => commands::markdown::run().await,
        Some(("pull", sub_matches)) => commands::pull::run(sub_matches),
        Some(("env", sub_matches)) => commands::env::run(sub_matches),
        Some(("info", _)) => commands::info::run(),
        Some(("version" | "v", _)) => commands::version::run().await,
        Some(("export", sub_matches)) => commands::export::run(sub_matches).await,
        Some(("eject", _)) => commands::eject::run(),
        _ => unreachable!(),
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
    async fn get_filename(
        &self,
        server: &model::Server,
        client: &reqwest::Client,
    ) -> Result<String>;
    async fn download(
        &self,
        server: &model::Server,
        client: &reqwest::Client,
        filename_hint: Option<&str>,
    ) -> Result<reqwest::Response>;
}

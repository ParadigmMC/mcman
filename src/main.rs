#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]

use anyhow::Result;
use clap::Command;
mod bootstrapper;
mod commands;
mod downloadable;
mod model;
mod util;

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
        .subcommand(commands::import::cli())
        .subcommand(commands::markdown::cli())
        .subcommand(commands::pull::cli())
        .subcommand(commands::info::cli())
        .subcommand(commands::version::cli())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => commands::init::run(sub_matches).await,
        Some(("build", sub_matches)) => commands::build::run(sub_matches).await,
        Some(("import" | "i", sub_matches)) => commands::import::run(sub_matches).await,
        Some(("markdown" | "md", _)) => commands::markdown::run().await,
        Some(("pull", sub_matches)) => commands::pull::run(sub_matches),
        Some(("info", _)) => commands::info::run(),
        Some(("version" | "v", _)) => commands::version::run().await,
        _ => unreachable!(),
    }
}

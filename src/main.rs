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
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(commands::build::cli())
        .subcommand(commands::init::cli())
        .subcommand(commands::version::cli())
        .subcommand(commands::markdown::cli())
        .subcommand(commands::info::cli())
        .subcommand(commands::pull::cli())
        .subcommand(commands::import::cli())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("build", sub_matches)) => commands::build::run(sub_matches).await,
        Some(("init", sub_matches)) => commands::init::run(sub_matches).await,
        Some(("import" | "i", sub_matches)) => commands::import::run(sub_matches).await,
        Some(("markdown" | "md", _)) => commands::markdown::run().await,
        Some(("info", _sub_matches)) => commands::info::run(),
        Some(("pull", sub_matches)) => commands::pull::run(sub_matches),
        Some(("version", _)) => commands::version::run().await,
        _ => unreachable!(),
    }
}

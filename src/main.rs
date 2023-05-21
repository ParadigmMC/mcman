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
        .subcommand(commands::setup::cli())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("build", sub_matches)) => commands::build::run(sub_matches).await,
        Some(("init", sub_matches)) => commands::init::run(sub_matches),
        Some(("setup", sub_matches)) => commands::setup::run(),
        Some(("version", sub_matches)) => {
            commands::version::run(sub_matches);
            Ok(())
        }
        _ => unreachable!(),
    }
}

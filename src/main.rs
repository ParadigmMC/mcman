#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

use clap::Command;
mod commands;
mod downloadable;
mod error;
mod model;

fn cli() -> Command {
    Command::new("mcman")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(commands::build::cli())
        .subcommand(commands::init::cli())
        .subcommand(commands::version::cli())
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    let res = match matches.subcommand() {
        Some(("build", sub_matches)) => commands::build::run(sub_matches).await,
        Some(("init", sub_matches)) => commands::init::run(sub_matches),
        Some(("version", sub_matches)) => commands::version::run(sub_matches),
        _ => unreachable!(),
    };

    if let Err(err) = res {
        println!("ERROR: {}", err);
        std::process::exit(1)
    }
}

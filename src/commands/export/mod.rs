use anyhow::Result;
use clap::{ArgMatches, Command};

mod mrpack;
mod packwiz;

pub fn cli() -> Command {
    Command::new("export")
        .about("Exporting tools")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(mrpack::cli())
        .subcommand(packwiz::cli())
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("mrpack", sub_matches)) => mrpack::run(sub_matches).await?,
        Some(("packwiz" | "pw", sub_matches)) => packwiz::run(sub_matches).await?,
        _ => unreachable!(),
    }
    Ok(())
}

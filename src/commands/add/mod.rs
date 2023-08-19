use anyhow::Result;
use clap::{ArgMatches, Command};

mod modrinth;

pub fn cli() -> Command {
    Command::new("add")
        .about("Add a plugin/mod/datapack")
        .subcommand(modrinth::cli())
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("modrinth" | "mr", sub_matches)) => modrinth::run(sub_matches).await?,
        _ => unreachable!(),
    }
    Ok(())
}

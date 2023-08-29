use anyhow::Result;
use clap::{ArgMatches, Command};

mod unpack;

pub fn cli() -> Command {
    Command::new("world")
        .about("Pack or unpack a world")
        .visible_alias("w")
        .subcommand(unpack::cli())
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("unpack" | "unzip", sub_matches)) => unpack::run(sub_matches).await?,
        _ => unreachable!(),
    }
    Ok(())
}

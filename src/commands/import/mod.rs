use anyhow::Result;
use clap::{Command, ArgMatches};

mod url;

pub fn cli() -> Command {
    Command::new("import")
        .about("Importing tools")
        .subcommand(url::cli())
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("url", sub_matches)) => url::run(sub_matches).await?,
        _ => unreachable!(),
    }
    Ok(())
}

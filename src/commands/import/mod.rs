use anyhow::Result;
use clap::{ArgMatches, Command};

mod url;

pub fn cli() -> Command {
    Command::new("import")
        .about("Importing tools")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(url::cli())
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("url", sub_matches)) => url::run(sub_matches).await?,
        _ => unreachable!(),
    }
    Ok(())
}

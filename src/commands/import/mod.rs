use anyhow::Result;
use clap::{ArgMatches, Command};

mod customs;
mod url;

pub fn cli() -> Command {
    Command::new("import")
        .about("Importing tools")
        .visible_alias("i")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(url::cli())
        .subcommand(customs::cli())
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("url", sub_matches)) => url::run(sub_matches).await?,
        Some(("customs", _)) => customs::run().await?,
        _ => unreachable!(),
    }
    Ok(())
}

use anyhow::Result;
use clap::{ArgMatches, Command};

mod customs;
mod datapack;
mod mrpack;
mod packwiz;
mod url;

pub fn cli() -> Command {
    Command::new("import")
        .about("Importing tools")
        .visible_alias("i")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(url::cli())
        .subcommand(datapack::cli())
        .subcommand(mrpack::cli())
        .subcommand(packwiz::cli())
        .subcommand(customs::cli())
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("url", sub_matches)) => url::run(sub_matches).await?,
        Some(("datapack" | "dp", sub_matches)) => datapack::run(sub_matches).await?,
        Some(("mrpack", sub_matches)) => mrpack::run(sub_matches).await?,
        Some(("packwiz" | "pw", sub_matches)) => packwiz::run(sub_matches).await?,
        Some(("customs", _)) => customs::run().await?,
        _ => unreachable!(),
    }
    Ok(())
}

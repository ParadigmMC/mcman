use anyhow::Result;
use clap::{ArgMatches, Command};

mod docker;
mod gitignore;

pub fn cli() -> Command {
    Command::new("env")
        .about("Helpers for setting up the environment")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(gitignore::cli())
        .subcommand(docker::cli())
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("gitignore", sub_matches)) => gitignore::run(sub_matches)?,
        Some(("docker", sub_matches)) => docker::run(sub_matches)?,
        _ => unreachable!(),
    }
    Ok(())
}

use clap::{ArgMatches, Command};

use crate::error::Result;

pub fn cli() -> Command {
    Command::new("build").about("Build using server.toml configuration")
}

pub async fn run(_matches: &ArgMatches) -> Result<()> {
    todo!();
}

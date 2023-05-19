use clap::{ArgMatches, Command};

use crate::error::Result;

pub fn cli() -> Command {
    Command::new("version").about("Show version information")
}

pub fn run(_matches: &ArgMatches) -> Result<()> {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Ok(())
}

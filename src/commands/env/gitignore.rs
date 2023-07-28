use anyhow::Result;
use clap::{ArgMatches, Command};
use console::style;

use crate::util::env::write_gitignore;

pub fn cli() -> Command {
    Command::new("gitignore")
        .about("Modify the gitignore")
}

pub fn run(_matches: &ArgMatches) -> Result<()> {
    let path = write_gitignore()?;

    println!(" > {} {}", style("Configured gitignore at").green(), style(path.to_string_lossy()).dim());

    Ok(())
}

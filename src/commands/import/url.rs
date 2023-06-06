use anyhow::{Context, Result};
use clap::{arg, ArgMatches, Command};
use std::path::Path;

use crate::model::Server;

pub fn cli() -> Command {
    Command::new("url")
        .about("Import from an URL")
        .arg(arg!(<url>).required(true))
        .arg(arg!(-m --mod "Explicitly define it as a mod").required(false))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let mut server =
        Server::load(Path::new("server.toml")).context("Failed to load server.toml")?;

    server
        .import_from_url(
            matches.get_one::<String>("url").unwrap(),
            matches.get_one::<bool>("mod").map(|&b| b.to_owned()),
        )?;
    
    server.save(Path::new("server.toml"))?;

    println!(" > Imported!");

    Ok(())
}

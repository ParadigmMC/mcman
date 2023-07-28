use anyhow::{Result, Context};
use clap::{ArgMatches, Command};
use console::style;

use crate::{util::env::{write_dockerfile, write_dockerignore}, model::Server};

pub fn cli() -> Command {
    Command::new("docker")
        .about("Write the default Dockerfile and .dockerignore")
}

pub fn run(_matches: &ArgMatches) -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;

    write_dockerfile(&server.path)
        .context("writing Dockerfile")?;
    write_dockerignore(&server.path)
        .context("writing .dockerignore")?;

    println!(" > {}", style("Default docker files were written successfully").dim());

    Ok(())
}

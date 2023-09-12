use anyhow::{Context, Result};
use console::style;

use crate::{
    model::Server,
    util::env::{write_dockerfile, write_dockerignore},
};

pub fn run() -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;

    write_dockerfile(&server.path).context("writing Dockerfile")?;
    write_dockerignore(&server.path).context("writing .dockerignore")?;

    println!(
        " > {}",
        style("Default docker files were written successfully").dim()
    );

    Ok(())
}

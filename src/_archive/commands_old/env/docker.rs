use anyhow::{Context, Result};

use crate::{
    app::App,
    util::env::{write_dockerfile, write_dockerignore},
};

pub fn run(app: &App) -> Result<()> {
    write_dockerfile(&app.server.path).context("writing Dockerfile")?;
    write_dockerignore(&app.server.path).context("writing .dockerignore")?;

    app.success("Default docker files were written successfully");

    Ok(())
}

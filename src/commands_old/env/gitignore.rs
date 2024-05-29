use anyhow::Result;

use crate::{app::App, util::env::write_git};

pub fn run(app: &App) -> Result<()> {
    write_git()?;

    app.success("Configured gitignore and gitattributes");

    Ok(())
}

use anyhow::Result;

use crate::{util::env::write_git, app::App};

pub fn run(app: App) -> Result<()> {
    write_git()?;

    app.success("Configured gitignore and gitattributes");

    Ok(())
}

use anyhow::Result;

use crate::{util::env::write_gitignore, app::App};

pub fn run(app: App) -> Result<()> {
    let path = write_gitignore()?;

    app.success(format!("Configured gitignore at {}", path.to_string_lossy()))?;

    Ok(())
}

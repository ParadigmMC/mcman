use anyhow::Result;
use console::style;

use crate::util::env::write_gitignore;

pub fn run() -> Result<()> {
    let path = write_gitignore()?;

    println!(
        " > {} {}",
        style("Configured gitignore at").green(),
        style(path.to_string_lossy()).dim()
    );

    Ok(())
}

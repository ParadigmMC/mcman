use std::{fs, path::Path};

use anyhow::Result;

use crate::{app::App, util::env::get_git_root};

pub fn run(app: &App) -> Result<()> {
    let path = Path::new(&get_git_root().unwrap_or(".".to_owned()))
        .join(".github")
        .join("workflows");

    fs::write(
        path.join("packwiz.yml"),
        include_bytes!("../../../res/workflows/packwiz.yml"),
    )?;

    app.success("packwiz.yml workflow created");

    Ok(())
}

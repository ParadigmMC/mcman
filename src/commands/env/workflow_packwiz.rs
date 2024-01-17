use std::{io::Write, path::Path};

use anyhow::Result;
use std::fs::File;

use crate::{app::App, util::env::get_git_root};

pub fn run(app: &App) -> Result<()> {
    let path = Path::new(&get_git_root().unwrap_or(".".to_owned()))
        .join(".github")
        .join("workflows");

    let mut f = File::create(path.join("packwiz.yml"))?;
    f.write_all(include_bytes!("../../../res/workflows/packwiz.yml"))?;
    app.success("packwiz.yml workflow created");

    Ok(())
}

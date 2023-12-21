use std::{path::Path, io::Write};

use anyhow::Result;
use std::fs::File;

use crate::{util::env::get_git_root, app::App};

pub fn run(app: &App) -> Result<()> {
    let path = Path::new(&get_git_root()?.unwrap_or(".".to_owned())).join(".github").join("workflows");

    let mut f = File::create(path.join("test.yml"))?;
    f.write_all(include_bytes!("../../../res/workflows/test.yml"))?;
    app.success("test.yml workflow created");

    Ok(())
}

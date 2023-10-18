use std::path::Path;

use anyhow::{Context, Result};
use indicatif::ProgressBar;
use tempfile::Builder;

use crate::app::App;

#[derive(clap::Args)]
pub struct Args {
    source: String,
}

pub async fn run(mut app: App, args: Args) -> Result<()> {
    let src = args.source;

    let tmp_dir = Builder::new().prefix("mcman-mrpack-import").tempdir()?;

    let f = if Path::new(&src).exists() {
        std::fs::File::open(&src)?
    } else {
        let dl = app.dl_from_string(&src).await?;
        let resolved = app.download(&dl, tmp_dir.path().to_path_buf(), ProgressBar::new_spinner()).await?;
        let path = tmp_dir.path().join(&resolved.filename);
        std::fs::File::open(path)?
    };

    app.mrpack().import_all(f, None).await?;

    app.server.save()?;
    app.refresh_markdown().await?;

    println!(" > Imported!");

    Ok(())
}

use std::path::Path;

use anyhow::Result;
use indicatif::ProgressBar;
use tempfile::Builder;

use crate::{app::App, interop::mrpack::MRPackReader, model::Downloadable};
use std::fs::File;

#[derive(clap::Args, Default)]
pub struct Args {
    source: String,
    #[arg(long)]
    keep: bool,
}

pub async fn run(mut app: App, args: Args) -> Result<()> {
    let src = args.source;

    let tmp_dir = Builder::new().prefix("mcman-mrpack-import").tempdir()?;

    let f = if Path::new(&src).exists() {
        File::open(&src)?
    } else {
        let dl = if src.starts_with("http") && src.ends_with(".mrpack") {
            Downloadable::Url {
                url: src,
                filename: None,
                desc: None,
            }
        } else {
            app.dl_from_string(&src).await?
        };
        let resolved = app
            .download(
                &dl,
                tmp_dir.path().to_path_buf(),
                ProgressBar::new_spinner(),
            )
            .await?;
        let path = tmp_dir.path().join(resolved.filename);
        File::open(path)?
    };

    if !args.keep {
        app.server.mods = vec![];
        app.info("cleared mods list");
    }

    app.mrpack()
        .import_all(MRPackReader::from_reader(f)?, None)
        .await?;

    app.save_changes()?;
    app.refresh_markdown().await?;

    Ok(())
}

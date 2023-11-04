use std::path::PathBuf;

use anyhow::Result;
use indicatif::ProgressBar;

use crate::app::App;

#[derive(clap::Args)]
pub struct Args {
    /// A downloadable to download
    downloadable: Option<String>,
}

pub async fn run(app: App, args: Args) -> Result<()> {
    let string = if let Some(t) = args.downloadable {
        t
    } else {
        app.prompt_string("What to download")?
    };

    let dl = app.dl_from_string(&string).await?;

    app.download(
        &dl,
        PathBuf::from("."),
        app.multi_progress.add(ProgressBar::new_spinner())
    ).await?;

    Ok(())
}

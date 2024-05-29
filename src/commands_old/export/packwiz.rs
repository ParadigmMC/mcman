use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::app::App;

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, short)]
    /// The output directory for the packwiz files
    output: Option<PathBuf>,
}

pub async fn run(mut app: App, args: Args) -> Result<()> {
    let default_output = app.server.path.join("pack");
    let output_dir = args.output.unwrap_or(default_output);

    fs::create_dir_all(&output_dir)?;

    app.packwiz().export_all(output_dir).await?;

    Ok(())
}

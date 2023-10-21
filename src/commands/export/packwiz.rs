use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::app::App;

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, short)]
    /// The output directory for the packwiz files
    output: Option<PathBuf>,
    #[arg(long)]
    /// Use edge.forgecdn.net instead of metadata:curseforge
    cfcdn: bool,
}

pub async fn run(app: App, args: Args) -> Result<()> {
    let default_output = app.server.path.join("pack");
    let output_dir = args.output.unwrap_or(default_output);

    let cf_usecdn = args.cfcdn;

    todo!();

    Ok(())
}

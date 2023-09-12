use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::{
    create_http_client,
    model::Server,
    util::packwiz::{export_packwiz, PackwizExportOptions},
};

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, short)]
    /// The output directory for the packwiz files
    output: Option<PathBuf>,
    #[arg(long)]
    /// Use edge.forgecdn.net instead of metadata:curseforge
    cfcdn: bool,
}

pub async fn run(args: Args) -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let default_output = server.path.join("pack");
    let output_dir = args.output.unwrap_or(default_output);

    let cf_usecdn = args.cfcdn;

    export_packwiz(
        &output_dir,
        &http_client,
        &server,
        &PackwizExportOptions { cf_usecdn },
    )
    .await?;

    Ok(())
}

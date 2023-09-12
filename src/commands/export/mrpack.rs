use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::{create_http_client, model::Server, util::mrpack::export_mrpack};

#[derive(clap::Args)]
pub struct Args {
    /// Export as filename
    filename: Option<PathBuf>,
    /// Set the version ID of the mrpack
    #[arg(long, short)]
    version: Option<String>,
}

pub async fn run(args: Args) -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let s = server
        .name
        .clone()
        .replace(|c: char| !c.is_alphanumeric(), "");

    let default_output =
        PathBuf::from(if s.is_empty() { "server".to_owned() } else { s } + ".mrpack");

    let output_filename = args.filename.unwrap_or(default_output);

    let output_filename = if output_filename.extension().is_none() {
        output_filename.with_extension("mrpack")
    } else {
        output_filename
    };

    let version_id = args.version;

    let output_file =
        std::fs::File::create(output_filename).context("Creating mrpack output file")?;

    export_mrpack(
        &http_client,
        &server,
        None,
        &version_id.unwrap_or("".to_string()),
        output_file,
    )
    .await?;

    Ok(())
}

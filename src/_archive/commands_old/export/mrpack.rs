use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::{app::App, interop::mrpack::MRPackWriter};
use std::fs::File;

#[derive(clap::Args)]
pub struct Args {
    /// Export as filename
    filename: Option<PathBuf>,
    /// Set the version ID of the mrpack
    #[arg(long, short)]
    version: Option<String>,
}

pub async fn run(mut app: App, args: Args) -> Result<()> {
    let s = app
        .server
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

    let output_file = File::create(output_filename).context("Creating mrpack output file")?;

    app.mrpack()
        .export_all(MRPackWriter::from_writer(output_file))
        .await?;

    Ok(())
}

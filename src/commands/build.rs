use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::{
    core::BuildContext,
    create_http_client,
    model::{Lockfile, Network, Server}, App,
};

#[derive(clap::Args)]
pub struct Args {
    /// The output directory for the server
    #[arg(short, long, value_name = "file")]
    output: Option<PathBuf>,
    /// Skip some stages
    #[arg(long, value_name = "stages")]
    skip: Vec<String>,
    #[arg(long)]
    /// Don't skip downloading already downloaded jars
    force: bool,
}

pub async fn run(app: App, args: Args) -> Result<()> {
    let default_output = app.server.path.join("server");
    let output_dir = args.output.unwrap_or(default_output);

    let lockfile = Lockfile::get_lockfile(&output_dir)?;

    let force = args.force;

    let skip_stages = args.skip;

    std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

    let mut ctx = BuildContext {
        app: &app,
        force,
        skip_stages,
        lockfile,
        output_dir,
        new_lockfile: Lockfile::default(),
        server_process: None,
    };

    ctx.build_all().await?;

    Ok(())
}

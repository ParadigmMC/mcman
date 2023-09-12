use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::{
    core::BuildContext,
    create_http_client,
    model::{Lockfile, Network, Server},
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

pub async fn run(args: Args) -> Result<BuildContext> {
    let server = Server::load().context("Failed to load server.toml")?;
    let network = Network::load()?;
    let http_client = create_http_client()?;

    let default_output = server.path.join("server");
    let output_dir = args.output.unwrap_or(default_output);

    let lockfile = Lockfile::get_lockfile(&output_dir)?;

    let force = args.force;

    let skip_stages = args.skip;

    std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

    let mut ctx = BuildContext {
        server,
        network,
        http_client,
        force,
        skip_stages,
        lockfile,
        output_dir,
        ..Default::default()
    };

    ctx.build_all().await?;

    Ok(ctx)
}

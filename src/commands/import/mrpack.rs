use anyhow::{Context, Result};
use tempfile::Builder;

use crate::{
    create_http_client,
    model::Server,
    util::mrpack::{import_from_mrpack, mrpack_source_to_file},
};

#[derive(clap::Args)]
pub struct Args {
    source: String,
}

pub async fn run(args: Args) -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let src = args.source;

    let tmp_dir = Builder::new().prefix("mcman-mrpack-import").tempdir()?;

    let f = mrpack_source_to_file(&src, &http_client, &tmp_dir, &server).await?;

    import_from_mrpack(&mut server, &http_client, f).await?;

    server.save()?;

    server.refresh_markdown(&http_client).await?;

    println!(" > Imported!");

    Ok(())
}

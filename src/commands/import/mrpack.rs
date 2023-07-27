use anyhow::{Context, Result};
use clap::{arg, ArgMatches, Command};
use tempfile::Builder;

use crate::{
    create_http_client,
    model::Server,
    util::mrpack::{import_from_mrpack, mrpack_source_to_file},
};

pub fn cli() -> Command {
    Command::new("mrpack")
        .about("Import from .mrpack (modrinth modpacks)")
        .arg(arg!(<source> "File or url").required(true))
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let src = matches.get_one::<String>("source").unwrap();

    let tmp_dir = Builder::new().prefix("mcman-mrpack-import").tempdir()?;

    let f = mrpack_source_to_file(src, &http_client, &tmp_dir, &server).await?;

    import_from_mrpack(&mut server, &http_client, f).await?;

    server.save()?;

    server.refresh_markdown(&http_client).await?;

    println!(" > Imported!");

    Ok(())
}

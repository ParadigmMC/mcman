use std::{path::PathBuf, fs::File};

use anyhow::{Context, Result};
use clap::{arg, ArgMatches, Command};
use console::style;
use tempfile::Builder;

use crate::{commands::version::APP_USER_AGENT, model::Server, util::{mrpack::{import_from_mrpack, resolve_mrpack_source}, download_with_progress}};

pub fn cli() -> Command {
    Command::new("mrpack")
        .about("Import from .mrpack (modrinth modpacks)")
        .arg(arg!(<source> "File or url").required(true))
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;

    let http_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .context("Failed to create HTTP client")?;

    let src = matches.get_one::<String>("source").unwrap();

    let tmp_dir = Builder::new().prefix("mcman-mrpack-import").tempdir()?;

    let filename = if src.starts_with("http") || src.starts_with("mr:") {
        let fname = tmp_dir.path().join("pack.mrpack");
        let file = tokio::fs::File::create(&fname).await?;
        
        let downloadable = resolve_mrpack_source(src, &http_client).await?;

        println!(" > {}", style("Downloading mrpack...").green());

        download_with_progress(
            file,
            &format!("Downloading {src}..."),
            &downloadable,
            &server,
            &http_client,
        ).await?;

        fname
    } else {
        PathBuf::from(src)
    };

    let f = File::open(filename).context("opening file")?;

    import_from_mrpack(&mut server, &http_client, f).await?;

    server.save()?;

    println!(" > Imported!");

    Ok(())
}

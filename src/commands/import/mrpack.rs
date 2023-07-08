use std::{path::PathBuf, fs::File};

use anyhow::{Context, Result, anyhow, bail};
use clap::{arg, ArgMatches, Command};
use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use reqwest::Url;
use tempfile::Builder;

use crate::{commands::version::APP_USER_AGENT, downloadable::{Downloadable, sources::modrinth::{fetch_modrinth_versions, ModrinthVersion}}, model::Server, util::{mrpack::import_from_mrpack, download_with_progress}};

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

    let filename = if src.starts_with("http") {
        println!(" > {}", style("Downloading mrpack...").green());

        let fname = tmp_dir.path().join("pack.mrpack");
        let file = tokio::fs::File::create(&fname).await?;

        let url = Url::parse(src)?;

        let modpack_id = {
            if url.domain() == Some("modrinth.com")
            && url.path().starts_with("/modpack") {
                url.path_segments()
                    .ok_or(anyhow!("Invalid modrinth /modpack URL"))?
                    .nth(1)
            } else {
                None
            }
        };

        let downloadable = if let Some(id) = modpack_id {
            let versions: Vec<ModrinthVersion> = fetch_modrinth_versions(&http_client, &id, None)
                    .await?
                    .into_iter()
                    .filter(|v| v.game_versions.contains(&server.mc_version))
                    .collect();

            let version = {
                if versions.is_empty() {
                    bail!("No compatible versions in modrinth project");
                }

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("  Which version?")
                    .default(0)
                    .items(
                        &versions
                            .iter()
                            .map(|v| {
                                let num = &v.version_number;
                                let name = &v.name;
                                let compat = v.loaders.join(",");
                                format!("[{num}] {name} / {compat}")
                            })
                            .collect::<Vec<String>>(),
                    )
                    .interact()
                    .unwrap();

                versions[selection].clone()
            };

            Downloadable::Modrinth {
                id: id.to_owned(),
                version: version.id,
            }
        } else {
            Downloadable::Url { url: src.clone(), filename: None, desc: None }
        };

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

use anyhow::{Context, Result};
use clap::{arg, ArgMatches, Command};
use console::style;

use crate::{create_http_client, model::Server, util::packwiz::packwiz_import_from_source};

pub fn cli() -> Command {
    Command::new("packwiz")
        .about("Import from packwiz")
        .visible_alias("pw")
        .arg(arg!(<source> "File or url").required(true))
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let src = matches.get_one::<String>("source").unwrap();

    println!(" > {}", style("Importing from packwiz pack.toml...").dim());

    let (_pack, mod_count, config_count) =
        packwiz_import_from_source(&http_client, src, &mut server).await?;

    server.save()?;

    println!(
        " > {} {} {} {} {}",
        style("Imported").green().bold(),
        style(mod_count).cyan(),
        style("mods and").green(),
        style(config_count).cyan(),
        style("config files").green(),
    );

    server.refresh_markdown(&http_client).await?;

    Ok(())
}

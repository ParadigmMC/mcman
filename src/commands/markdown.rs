use console::style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use indexmap::IndexMap;
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;

use crate::commands::version::APP_USER_AGENT;
use crate::model::Server;
use crate::util::md::MarkdownTable;
use anyhow::{Context, Result};
use clap::Command;

pub fn cli() -> Command {
    Command::new("markdown")
        .about("Update markdown files with server info")
        .visible_alias("md")
}

pub static NOTICE: &str = "";
pub static SERVERINFO_REGEX: &str =
    r"(<!--start:mcman-server-->)([\w\W]*)(<!--end:mcman-server-->)";
pub static SERVERINFO_START: &str = "<!--start:mcman-server-->";
pub static SERVERINFO_END: &str = "<!--end:mcman-server-->";

pub static ADDONS_REGEX: &str = r"(<!--start:mcman-addons-->)([\w\W]*)(<!--end:mcman-addons-->)";
pub static ADDONS_START: &str = "<!--start:mcman-addons-->";
pub static ADDONS_END: &str = "<!--end:mcman-addons-->";

pub async fn run() -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;
    let http_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .context("Failed to create HTTP client")?;

    if server.markdown.files.is_empty() {
        println!(" ! {}", style("No markdown files were found").yellow());

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add and use README.md?")
            .interact()?
        {
            server.markdown.files.push("README.md".to_owned());
            server.save()?;
        } else {
            return Ok(());
        }
    }

    update_files(&http_client, &server).await?;

    Ok(())
}

pub async fn update_files(http_client: &reqwest::Client, server: &Server) -> Result<()> {
    println!(" > {}", style("Updating markdown files...").dim());

    let server_info_text = {
        let table = create_table_server(server);

        SERVERINFO_START.to_owned() + NOTICE + "\n" + &table.render() + "\n" + SERVERINFO_END
    };

    let addons_table = create_table_addons(http_client, server).await?;

    let addon_list_text =
        { ADDONS_START.to_owned() + NOTICE + "\n" + &addons_table.render() + "\n" + ADDONS_END };

    let serv_regex = Regex::new(SERVERINFO_REGEX).unwrap();
    let addon_regex = Regex::new(ADDONS_REGEX).unwrap();

    let len = server.markdown.files.len();
    for (idx, filename) in server.markdown.files.iter().enumerate() {
        let path = server.path.join(filename);

        if !path.exists() {
            println!(
                "   ({idx:w$}/{len}) {}: {filename}",
                style("File not found: ").red(),
                w = len.to_string().len()
            );
            continue;
        }

        let file_content = fs::read_to_string(&path)?;

        let stage1 = serv_regex
            .replace_all(&file_content, |_caps: &regex::Captures| {
                server_info_text.clone()
            })
            .into_owned();

        let stage2 =
            addon_regex.replace_all(&stage1, |_caps: &regex::Captures| addon_list_text.clone());

        let mut f = File::create(&path)?;
        f.write_all(stage2.as_bytes())?;

        println!(
            "   ({idx:w$}/{len}) Updated {}!",
            style(filename).green(),
            w = len.to_string().len()
        );
    }

    Ok(())
}

pub fn create_table_server(server: &Server) -> MarkdownTable {
    let mut map = IndexMap::new();

    map.insert("Version".to_owned(), server.mc_version.clone());
    map.insert("Type".to_owned(), server.jar.get_md_link());

    if let Some(extra) = server.jar.get_extra_jar_map() {
        map.extend(extra);
    }

    MarkdownTable::from_map(&map)
}

pub async fn create_table_addons(
    http_client: &reqwest::Client,
    server: &Server,
) -> Result<MarkdownTable> {
    let mut table = MarkdownTable::new();

    for plugin in &server.plugins {
        table.add_from_map(&plugin.fetch_info_to_map(http_client).await?);
    }

    for addon in &server.mods {
        table.add_from_map(&addon.fetch_info_to_map(http_client).await?);
    }

    Ok(table)
}

pub fn create_table_server_console(server: &Server) -> MarkdownTable {
    let mut map = IndexMap::new();

    map.insert("Name".to_owned(), server.name.clone());
    map.insert("Version".to_owned(), server.mc_version.clone());
    map.insert("Type".to_owned(), server.jar.get_type_name());

    if let Some(extra) = server.jar.get_extra_jar_map() {
        map.extend(extra);
    }

    MarkdownTable::from_map(&map)
}

pub fn initialize_readme(server: &Server) -> Result<()> {
    let mut f = File::create("./README.md")?;
    let readme_content = include_str!("../../res/default_readme");
    let readme_content = readme_content
        .replace("{SERVER_NAME}", &server.name)
        .replace(
            "{ADDON_HEADER}",
            if server.jar.is_modded() {
                "Mods"
            } else {
                "Plugins"
            },
        );

    f.write_all(readme_content.as_bytes())?;

    Ok(())
}

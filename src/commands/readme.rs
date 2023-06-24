use console::style;
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::vec;

use crate::commands::version::APP_USER_AGENT;
use crate::downloadable::Downloadable;
use crate::model::Server;
use anyhow::{Context, Result};
use clap::Command;

pub fn cli() -> Command {
    Command::new("readme").about("Update or create README.md with server info")
}

// TODO: pretty print the tables...

pub static NOTICE: &str = "";
pub static SERVERINFO_REGEX: &str =
    r"(<!--start:mcman-server-->)([\w\W]*)(<!--end:mcman-server-->)";
pub static SERVERINFO_START: &str = "<!--start:mcman-server-->";
pub static SERVERINFO_END: &str = "<!--end:mcman-server-->";

pub static ADDONS_REGEX: &str = r"(<!--start:mcman-addons-->)([\w\W]*)(<!--end:mcman-addons-->)";
pub static ADDONS_START: &str = "<!--start:mcman-addons-->";
pub static ADDONS_END: &str = "<!--end:mcman-addons-->";

pub async fn run() -> Result<()> {
    let server = Server::load(Path::new("server.toml")).context("Failed to load server.toml")?;
    let http_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .context("Failed to create HTTP client")?;

    if !PathBuf::from("./README.md").exists() {
        println!(" > {}", style("Creating README.md...").green());
        initialize_readme(&server)?;
    }

    println!(" > {}", style("Creating tables...").cyan());

    let server_info_text = {
        let mut line1: Vec<String> = vec![];
        let mut line2: Vec<String> = vec![];

        line1.push("Version".to_owned());
        line2.push(server.mc_version.clone());

        line1.push("Type".to_owned());
        line2.push(server.jar.get_type_str());

        let text = "| ".to_owned()
            + &line1.join(" | ")
            + " |"
            + "\n"
            + "| "
            + &line1
                .iter()
                .map(|_| "---")
                .collect::<Vec<&str>>()
                .join(" | ")
            + " |"
            + "\n"
            + "| "
            + &line2.join(" | ")
            + " |\n";

        SERVERINFO_START.to_owned() + NOTICE + "\n" + &text + SERVERINFO_END
    };

    let wrap = |r: Vec<String>| "| ".to_owned() + &r.join(" | ") + " |";
    let wrap_sep = |r: Vec<String>| {
        "| ".to_owned() + &r.iter().map(|_| "---").collect::<Vec<&str>>().join(" | ") + " |"
    };

    let addon_list_text = {
        let header: Vec<String> = vec!["Name".to_string(), "Description".to_string()];
        let mut rows: Vec<Vec<String>> = vec![];

        for plugin in &server.plugins {
            rows.push(plugin.fetch_str_row(&http_client).await?);
        }

        for addon in &server.mods {
            rows.push(addon.fetch_str_row(&http_client).await?);
        }

        let mut text = String::new();

        text += &wrap(header.clone());
        text += "\n";
        text += &wrap_sep(header);
        text += "\n";

        for row in rows {
            text += &wrap(row);
            text += "\n";
        }

        ADDONS_START.to_owned() + NOTICE + "\n" + &text + ADDONS_END
    };

    println!(" > {}", style("Updating README.md...").cyan());

    let readme_content = fs::read_to_string("./README.md")?;

    let serv_regex = Regex::new(SERVERINFO_REGEX).unwrap();

    let stage1 = serv_regex
        .replace_all(&readme_content, |_caps: &regex::Captures| {
            server_info_text.clone()
        })
        .into_owned();

    let addon_regex = Regex::new(ADDONS_REGEX).unwrap();

    let stage2 =
        addon_regex.replace_all(&stage1, |_caps: &regex::Captures| addon_list_text.clone());

    let mut f = File::create("./README.md")?;
    f.write_all(stage2.as_bytes())?;

    println!(" > {}", style("README.md updated successfully!").green());

    Ok(())
}

pub fn initialize_readme(server: &Server) -> Result<()> {
    let mut f = File::create("./README.md")?;
    let readme_content = include_str!("../../res/default_readme");
    let readme_content = readme_content
        .replace("{SERVER_NAME}", &server.name)
        .replace(
            "{ADDON_HEADER}",
            match server.jar {
                Downloadable::Quilt { .. } | Downloadable::Fabric { .. } => "Mods",
                _ => "Plugins",
            },
        );

    f.write_all(readme_content.as_bytes())?;

    Ok(())
}

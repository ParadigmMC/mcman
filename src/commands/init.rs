use console::style;
use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::ffi::OsStr;

use crate::commands::markdown;
use crate::{
    commands::version::APP_USER_AGENT,
    downloadable::{sources::vanilla::fetch_latest_mcver, Downloadable},
    model::{Server, ServerLauncher},
};
use anyhow::{bail, Context, Result};
use clap::{arg, ArgMatches, Command};

pub fn cli() -> Command {
    Command::new("init")
        .about("Initializes a new MCMan-powered Minecraft server")
        .arg(arg!(--name <NAME> "The name of the server").required(false))
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let http_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .context("Failed to create HTTP client")?;

    println!(" > {}", style("Initializing new server...").cyan());

    let res = std::fs::metadata("server.toml");
    if let Err(err) = res {
        if err.kind() != std::io::ErrorKind::NotFound {
            Err(err)?;
        }
    } else {
        bail!("server.toml already exists");
    }

    let current_dir = std::env::current_dir()?;
    let name = matches.get_one::<String>("name");
    let name = if let Some(name) = name {
        name.clone()
    } else {
        current_dir
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("")
            .to_owned()
    };

    let theme = ColorfulTheme::default();

    let name = Input::<String>::with_theme(&theme)
        .with_prompt("Server name?")
        .default(name.clone())
        .with_initial_text(&name)
        .interact_text()?;

    let serv_type = Select::with_theme(&theme)
        .with_prompt("Type of server?")
        .default(0)
        .items(&[
            "Normal Server (vanilla, spigot, paper etc.)",
            "Modded Server (forge, fabric, quilt etc.)",
            "Proxy Server (velocity, bungeecord, waterfall etc.)",
        ])
        .interact()?;

    let is_proxy = serv_type == 2;

    let mc_version = if is_proxy {
        "latest".to_owned()
    } else {
        let latest_ver = fetch_latest_mcver(&http_client)
            .await
            .context("Fetching latest version")?;

        Input::with_theme(&theme)
            .with_prompt("Server version?")
            .default(latest_ver)
            .interact_text()?
    };

    let launcher = if is_proxy {
        ServerLauncher {
            proxy_flags: true,
            aikars_flags: false,
            nogui: false,
            ..Default::default()
        }
    } else {
        ServerLauncher::default()
    };

    let jar = match serv_type {
        0 => Downloadable::select_jar_interactive(),
        1 => Downloadable::select_modded_jar_interactive(),
        2 => Downloadable::select_proxy_jar_interactive(),
        _ => unreachable!(),
    }?;

    let server = Server {
        name,
        mc_version,
        jar,
        launcher,
        ..Default::default()
    };

    initialize_environment(is_proxy).context("Initializing environment")?;
    server.save()?;

    let write_readme = if Path::new("./README.md").exists() {
        Confirm::with_theme(&theme)
            .default(true)
            .with_prompt("Overwrite README.md?")
            .interact()?
    } else {
        true
    };

    if write_readme {
        markdown::initialize_readme(&server).context("Initializing readme")?;
    }

    println!(" > {}", style("Server has been initialized!").cyan());
    println!(
        " > {} {}",
        style("Build using").cyan(),
        style("mcman build").bold()
    );

    Ok(())
}

pub fn initialize_environment(is_proxy: bool) -> Result<()> {
    std::fs::create_dir_all("./config")?;

    let mut f = File::create(".dockerignore")?;
    f.write_all(include_bytes!("../../res/default_dockerignore"))?;

    let mut f = File::create(".gitignore")?;
    f.write_all(include_bytes!("../../res/default_gitignore"))?;

    let mut f = File::create("Dockerfile")?;
    f.write_all(include_bytes!("../../res/default_dockerfile"))?;

    if !is_proxy {
        let mut f = File::create("./config/server.properties")?;
        f.write_all(include_bytes!("../../res/server.properties"))?;
    }

    Ok(())
}

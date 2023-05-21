use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::{arg, value_parser, ArgMatches, Command, ArgAction};
use console::{style, Style};

use super::version::APP_USER_AGENT;
use crate::{
    bootstrapper::{bootstrap, BootstrapContext},
    model::Server,
    util,
};

pub fn cli() -> Command {
    Command::new("build")
        .about("Build using server.toml configuration")
        .arg(
            arg!(-o --output <FILE> "The output directory for the server")
                .default_value("server")
                .value_parser(value_parser!(PathBuf)),
        )
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let mut server =
        Server::load(Path::new("server.toml")).context("Failed to load server.toml")?;
    let http_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .context("Failed to create HTTP client")?;
    let output_dir = matches.get_one::<PathBuf>("output").unwrap();
    std::fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    //let term = Term::stdout();
    let title = Style::new().blue().bold();
    
    println!(" Building {}...", style(server.name.clone()).green().bold());
    
    // stage 1: server jar
    println!(" stage 1: {}", title.apply_to("Server Jar"));
    
    let serverjar_name = server.jar.get_filename();
    if output_dir.join(serverjar_name.clone()).exists() {
        println!(
            "          Skipping server jar ({})",
            style(serverjar_name.clone()).dim()
        );
    } else {
        println!(
            "          Downloading server jar ({})",
            style(serverjar_name.clone()).dim()
        );

        util::download_with_progress(
            output_dir,
            &server.jar.get_filename(),
            server.jar,
            &http_client,
        )
        .await
        .context("Failed to download server jar")?;
    }

    // stage 2: plugins

    println!(" stage 2: {}", title.apply_to("Plugins"));

    if server.plugins.is_empty() {
        println!(
            "          {}",
            style("This server doesn't have any plugins, skipping").dim()
        );
    } else {
        let plugin_count = server.plugins.len();
        println!(
            "          {}",
            style(format!("{} plugins present, processing...", plugin_count)).dim()
        );

        std::fs::create_dir_all(output_dir.join("plugins")).context("Failed to create plugins directory")?;

        let mut i = 0;
        for plugin in server.plugins {
            i += 1;

            let plugin_name = plugin.get_filename();
            if output_dir.join("plugins").join(&plugin_name).exists() {
                println!(
                    "          ({}/{}) Skipping    : {}",
                    i,
                    plugin_count,
                    style(&plugin_name).dim()
                );
            } else {
                println!(
                    "          ({}/{}) Downloading : {}",
                    i,
                    plugin_count,
                    style(&plugin_name).dim()
                );
        
                util::download_with_progress(
                    &output_dir.join("plugins"),
                    &plugin_name,
                    plugin,
                    &http_client,
                )
                .await
                .context(format!("Failed to download plugin {plugin_name}"))?;

                println!(
                    "          ({}/{}) {}  : {}",
                    i,
                    plugin_count,
                    style("Downloaded").green().bold(),
                    style(&plugin_name).dim()
                );
            }
        }
    }

    // stage 3: bootstrap

    println!(" stage 3: {}", title.apply_to("Configurations"));

    let mut vars = HashMap::new();

    // TODO: read from .env file
    // TODO: environment variables

    for (key, value) in &server.variables {
        vars.insert(key.clone(), value.clone());
    }

    vars.insert("SERVER_NAME".to_owned(), server.name.clone());
    vars.insert("SERVER_VERSION".to_owned(), server.mc_version.clone());

    bootstrap(&BootstrapContext {
        vars,
        output_dir: output_dir.clone(),
    })?;

    println!("          {}", style("Bootstrapping complete").dim());

    // stage 4: launcher scripts

    println!(" stage 4: {}", title.apply_to("Scripts"));

    fs::write(
        output_dir.join("start.bat"),
        server
            .launcher
            .generate_script_win(&serverjar_name.clone(), &server.name),
    )?;

    let mut file;
    #[cfg(target_family = "unix")]
    {
        use os::unix::prelude::OpenOptionsExt;
        file = OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o755)
            .open(output_dir.join("start.sh"))?;
    }
    #[cfg(not(target_family = "unix"))]
    {
        file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(output_dir.join("start.sh"))?;
    }

    file.write_all(
        server
            .launcher
            .generate_script_linux(&serverjar_name, &server.name)
            .as_bytes(),
    )?;

    println!(
        "          {}",
        style("start.bat and start.sh created").dim()
    );

    Ok(())
}

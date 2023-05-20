use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::{arg, value_parser, ArgMatches, Command};

use crate::{model::Server, util};

use super::version::APP_USER_AGENT;

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

    let serverjar_name = server.jar.get_server_filename();

    // stage 1: server jar

    util::download_with_progress(output_dir, "server.jar", server.jar, &http_client)
        .await
        .context("Failed to download server jar")?;

    // stage 2: plugins

    // todo

    // stage 3: bootstrap

    // stage 4: launcher scripts

    fs::write(
        output_dir.join("start.bat"),
        server
            .launcher
            .generate_script_win(&serverjar_name, &server.name),
    )?;
    fs::write(
        output_dir.join("start.sh"),
        server
            .launcher
            .generate_script_linux(&serverjar_name, &server.name),
    )?;

    Ok(())
}

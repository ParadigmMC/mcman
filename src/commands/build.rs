use std::path::{Path, PathBuf};

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
    let server = Server::load(Path::new("server.toml")).context("Failed to load server.toml")?;
    let http_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .context("Failed to create HTTP client")?;
    let output_dir = matches.get_one::<PathBuf>("output").unwrap();
    std::fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    util::download_with_progress(output_dir, "server.jar", server.jar, &http_client)
        .await
        .context("Failed to download server jar")?;
    Ok(())
}

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{arg, value_parser, ArgMatches, Command};

use crate::{create_http_client, model::Server, util::mrpack::export_mrpack};

pub fn cli() -> Command {
    Command::new("mrpack")
        .about("Export as an mrpack")
        .arg(
            arg!([filename] "Export as filename")
                .value_parser(value_parser!(PathBuf))
                .required(false),
        )
        .arg(arg!(-v --version <version> "Set the version ID of the mrpack"))
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let s = server
        .name
        .clone()
        .replace(|c: char| !c.is_alphanumeric(), "");

    let default_output =
        PathBuf::from(if s.is_empty() { "server".to_owned() } else { s } + ".mrpack");

    let output_filename = matches
        .get_one::<PathBuf>("filename")
        .unwrap_or(&default_output)
        .clone();

    let output_filename = if output_filename.extension().is_none() {
        output_filename.with_extension("mrpack")
    } else {
        output_filename
    };

    let version_id = matches.get_one::<String>("version");

    let output_file =
        std::fs::File::create(output_filename).context("Creating mrpack output file")?;

    export_mrpack(
        &http_client,
        &server,
        None,
        version_id.unwrap_or(&String::new()),
        output_file,
    )
    .await?;

    Ok(())
}

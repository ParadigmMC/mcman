use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{arg, value_parser, ArgMatches, Command};

use crate::{
    create_http_client,
    model::Server,
    util::packwiz::{export_packwiz, PackwizExportOptions},
};

pub fn cli() -> Command {
    Command::new("packwiz")
        .visible_alias("pw")
        .about("Export packwiz")
        .arg(
            arg!(-o --output [FILE] "The output directory for the packwiz files")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(--cfcdn "Use edge.forgecdn.net instead of metadata:curseforge"))
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let default_output = server.path.join("pack");
    let output_dir = matches
        .get_one::<PathBuf>("output")
        .unwrap_or(&default_output)
        .clone();

    let cf_usecdn = matches.get_flag("cfcdn");

    export_packwiz(
        &output_dir,
        &http_client,
        &server,
        &PackwizExportOptions { cf_usecdn },
    )
    .await?;

    Ok(())
}

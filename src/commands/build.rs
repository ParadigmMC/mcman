use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{arg, value_parser, ArgMatches, Command};

use crate::{core::BuildContext, create_http_client, model::Server};

pub fn cli() -> Command {
    Command::new("build")
        .about("Build using server.toml configuration")
        .arg(
            arg!(-o --output [FILE] "The output directory for the server")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(--skip [stages] "Skip some stages").value_delimiter(','))
        .arg(arg!(--force "Don't skip downloading already downloaded jars"))
}

pub async fn run(matches: &ArgMatches) -> Result<BuildContext> {
    let server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let default_output = server.path.join("server");
    let output_dir = matches
        .get_one::<PathBuf>("output")
        .unwrap_or(&default_output)
        .clone();

    let force = matches.get_flag("force");

    let skip_stages = matches
        .get_many::<String>("skip")
        .map(|o| o.cloned().collect::<Vec<String>>())
        .unwrap_or(vec![]);

    std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

    let mut ctx = BuildContext {
        server,
        http_client,
        output_dir,
        force,
        skip_stages,
        ..Default::default()
    };

    ctx.build_all().await?;

    Ok(ctx)
}

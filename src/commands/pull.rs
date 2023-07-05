use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use clap::{arg, value_parser, ArgMatches, Command};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm};
use pathdiff::diff_paths;

use crate::{bootstrapper::BootstrapContext, model::Server};

pub fn cli() -> Command {
    Command::new("pull")
        .about("Pull a config file from server/ to config/")
        .arg(
            arg!(<file> "File to pull")
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;

    let path = matches.get_one::<PathBuf>("file").unwrap();
    let cannon = fs::canonicalize(path).context("Resolving absolute path of file")?;
    let diff =
        diff_paths(&cannon, fs::canonicalize(&server.path)?).ok_or(anyhow!("Cannot diff paths"))?;

    if !diff.starts_with("server") {
        bail!("You aren't inside server/");
    }

    // i got lazy ok

    let cx = BootstrapContext {
        output_dir: server.path.join("config"),
        vars: HashMap::new(),
    };

    let destination = cx.get_output_path(&diff);

    fs::create_dir_all(destination.parent().unwrap()).context("Failed to create dirs")?;

    if destination.exists()
        && !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "File '{}' already exists, overwrite?",
                destination.to_string_lossy()
            ))
            .default(false)
            .interact()?
    {
        return Ok(());
    }

    fs::copy(&cannon, &destination)?;

    println!(
        " {} => {}",
        style(&diff.to_string_lossy()).dim(),
        style(
            diff_paths(
                fs::canonicalize(&destination)?,
                fs::canonicalize(&server.path)?
            )
            .unwrap_or_default()
            .to_string_lossy()
        )
        .dim(),
    );

    Ok(())
}

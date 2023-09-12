use std::{fs, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm};
use glob::glob;
use pathdiff::diff_paths;

use crate::model::Server;

#[derive(clap::Args)]
pub struct Args {
    /// Files to pull
    #[arg(required = true)]
    file: String,
}

pub fn run(args: Args) -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;

    let files = args.file;

    for entry in glob(&files)? {
        let entry = entry?;

        let diff = diff_paths(&entry, fs::canonicalize(&server.path)?)
            .ok_or(anyhow!("Cannot diff paths"))?;

        if !diff.starts_with("server") {
            bail!("You aren't inside server/");
        }

        let mut destination = PathBuf::new();
        let mut iter = diff.components();
        iter.next().expect("Path to have atleast 1 component");
        destination.push(&server.path);
        destination.push("config");
        destination.extend(iter);

        fs::create_dir_all(destination.parent().unwrap()).context("Failed to create dirs")?;

        if destination.exists()
            && !Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "File '{}' already exists, overwrite?",
                    destination.display()
                ))
                .default(false)
                .interact()?
        {
            continue;
        }

        fs::copy(&entry, &destination)?;

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
    }

    Ok(())
}

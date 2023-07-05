use std::cmp::Ordering;

use anyhow::{Context, Result};
use clap::Command;
use console::style;
use semver::Version;

use crate::downloadable::sources::github;

pub fn cli() -> Command {
    Command::new("version").about("Show version information")
}

pub const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " - ",
    env!("CARGO_PKG_REPOSITORY"),
);

pub async fn run() -> Result<()> {
    println!(
        " > {} by {}",
        style(env!("CARGO_PKG_NAME")).green().bold(),
        style(env!("CARGO_PKG_AUTHORS")).magenta().bold()
    );
    println!("   version {}", style(env!("CARGO_PKG_VERSION")).bold());

    println!();

    println!(" {}", style("> checking for updates...").dim());

    let http_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .context("Failed to create HTTP client")?;

    let repo_name: String = env!("CARGO_PKG_REPOSITORY").chars().skip(19).collect();

    let releases = github::fetch_github_releases(&repo_name, &http_client).await?;

    let latest_ver = Version::parse(&releases.first().unwrap().tag_name)?;

    match Version::parse(env!("CARGO_PKG_VERSION"))?.cmp(&latest_ver) {
        Ordering::Equal => {
            println!(" > up to date!");
        }
        Ordering::Greater => {
            println!(" {}", style("> version is newer (dev/unreleased)").yellow());
        }
        Ordering::Less => {
            println!(" {}", style("> A new version is available!").cyan());
            println!(
                " {} {} => {}",
                style("|").cyan(),
                style(env!("CARGO_PKG_VERSION")).red(),
                style(&latest_ver).green().bold(),
            );
            println!(
                " {} {}",
                style("|").cyan(),
                env!("CARGO_PKG_REPOSITORY").to_owned()
                    + "/releases/tag/"
                    + &latest_ver.to_string()
            );
        }
    }

    Ok(())
}

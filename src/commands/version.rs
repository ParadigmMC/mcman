use std::cmp::Ordering;

use anyhow::Result;
use console::style;
use semver::Version;

use crate::{create_http_client, sources::github};

pub async fn run() -> Result<()> {
    println!(
        " > {} by {}",
        style(env!("CARGO_PKG_NAME")).green().bold(),
        style(env!("CARGO_PKG_AUTHORS")).magenta().bold()
    );
    println!("   version {}", style(env!("CARGO_PKG_VERSION")).bold());

    println!();

    println!(" {}", style("> checking for updates...").dim());

    let http_client = create_http_client()?;

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

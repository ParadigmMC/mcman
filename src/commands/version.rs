use std::cmp::Ordering;

use anyhow::Result;
use console::style;
use semver::Version;

use crate::app::BaseApp;

#[derive(clap::Args)]
pub struct Args {
    /// Only print the version
    #[arg(long)]
    pub plain: bool,
}

pub async fn run(base_app: BaseApp, args: Args) -> Result<()> {
    if args.plain {
        println!("{}", env!("CARGO_PKG_VERSION"));
    } else {
        println!(
            " > {} by {}\n   version {}\n\n {}",
            style(env!("CARGO_PKG_NAME")).green().bold(),
            style(env!("CARGO_PKG_AUTHORS")).magenta().bold(),
            style(env!("CARGO_PKG_VERSION")).bold(),
            style("> checking for updates...").dim()
        );

        let repo_name: String = env!("CARGO_PKG_REPOSITORY").chars().skip(19).collect();

        let app = base_app.upgrade_with_default_server()?;

        let releases = app.github().fetch_releases(&repo_name).await?;

        let latest_ver = Version::parse(&releases.first().unwrap().tag_name)?;

        match Version::parse(env!("CARGO_PKG_VERSION"))?.cmp(&latest_ver) {
            Ordering::Equal => {
                println!(" > up to date!");
            }
            Ordering::Greater => {
                println!(" {}", style("> version is newer (dev/unreleased)").yellow());
            }
            Ordering::Less => {
                println!(
                    " {}\n {} {} => {}\n {} {}",
                    style("> A new version is available!").cyan(),
                    style("|").cyan(),
                    style(env!("CARGO_PKG_VERSION")).red(),
                    style(&latest_ver).green().bold(),
                    style("|").cyan(),
                    env!("CARGO_PKG_REPOSITORY").to_owned()
                        + "/releases/tag/"
                        + &latest_ver.to_string()
                );
            }
        }
    }

    Ok(())
}

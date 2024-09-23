use std::sync::Arc;

use anyhow::Result;
use console::style;

use crate::api::{app::App, tools::java::get_java_installations};

#[derive(clap::Args)]
pub struct Args {
    #[arg(long)]
    pub json: bool,
}

pub async fn run(_app: Arc<App>, args: Args) -> Result<()> {
    let installs = get_java_installations().await;

    if args.json {
        println!("{}", serde_json::to_string(&installs)?);
        return Ok(());
    }

    for install in installs {
        println!(
            "{}",
            style(format!(
                "Java {}, {}",
                install.version, install.architecture
            ))
            .cyan()
            .bold()
        );

        println!("  Path: {}", style(install.path.display()).dim());
        println!("  Vendor: {}", install.vendor);
    }

    Ok(())
}

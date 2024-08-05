use std::sync::Arc;

use anyhow::Result;
use console::style;

use crate::api::{app::App, models::{ModpackSource, SourceType}};

#[derive(clap::Args)]
pub struct Args {}

pub async fn run(app: Arc<App>, args: Args) -> Result<()> {
    let sources = app.collect_sources().await?;

    for (idx, (base, source)) in sources.iter().enumerate() {
        println!(
            "{} {}{}",
            style(idx.to_string() + ".").cyan().bold(),
            style(source.source_name()).bold(),
            match source.source_type {
                SourceType::Modpack { modpack_type, .. } => format!(
                    "/{}", style(modpack_type.to_string()).bold()
                ),
                _ => String::new(),
            }
        );

        println!("   -> {}", style(source.accessor(base)?.to_string()).dim())
    }

    Ok(())
}

use std::sync::Arc;

use anyhow::Result;
use console::style;

use crate::api::{app::App, models::SourceType};

#[derive(clap::Args)]
pub struct Args {
    #[arg(short = 'a', long)]
    pub with_addons: bool,
}

pub async fn run(app: Arc<App>, args: Args) -> Result<()> {
    if let Some((base, network)) = &*app.network.read().await {
        println!("{} {}", style("Network:").bold(), network.name);
        println!("   -> {:?}", style(base.parent().unwrap()).dim());
        println!();
    };

    if let Some((base, server)) = &*app.server.read().await {
        println!("{} {}", style("Server:").bold(), server.name);
        println!("   -> {:?}", style(base.parent().unwrap()).dim());
    };

    println!();

    let sources = app.collect_sources().await?;

    for (idx, (base, source)) in sources.iter().enumerate() {
        println!(
            "{} {}{}",
            style(idx.to_string() + ".").cyan().bold(),
            style(source.source_name()).bold(),
            match source.source_type {
                SourceType::Modpack { modpack_type, .. } =>
                    format!("/{}", style(modpack_type.to_string()).bold()),
                _ => String::new(),
            }
        );

        println!("   -> {}", style(source.accessor(base)?.to_string()).dim());

        if args.with_addons {
            for (idx, addon) in source.resolve_addons(&app, base).await?.iter().enumerate() {
                println!(
                    "   {}. {} {}",
                    style(idx).bold(),
                    addon.addon_type,
                    style(addon.target.as_str()).dim()
                );
            }
        }
    }

    Ok(())
}

use anyhow::Result;

use crate::app::App;

mod datapack;
mod mrpack;
mod packwiz;
mod url;

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Import from a URL
    Url(url::Args),
    /// Import datapack from url
    #[command(visible_alias = "dp")]
    Datapack(datapack::Args),
    /// Import from .mrpack (modrinth modpacks)
    Mrpack(mrpack::Args),
    /// Import from packwiz
    #[command(visible_alias = "pw")]
    Packwiz(packwiz::Args),
}

impl Default for Commands {
    fn default() -> Self {
        Self::Url(url::Args::default())
    }
}

pub async fn run(app: App, subcommand: Commands) -> Result<()> {
    match subcommand {
        Commands::Url(args) => url::run(app, args).await?,
        Commands::Datapack(args) => datapack::run(app, args).await?,
        Commands::Mrpack(args) => mrpack::run(app, args).await?,
        Commands::Packwiz(args) => packwiz::run(app, args).await?,
    }
    Ok(())
}

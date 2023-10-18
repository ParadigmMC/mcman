use anyhow::Result;

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

pub async fn run(subcommands: Commands) -> Result<()> {
    match subcommands {
        Commands::Url(args) => url::run(args).await?,
        Commands::Datapack(args) => datapack::run(args).await?,
        Commands::Mrpack(args) => mrpack::run(args).await?,
        Commands::Packwiz(args) => packwiz::run(args).await?,
    }
    Ok(())
}

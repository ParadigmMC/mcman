use anyhow::Result;

use crate::app::App;

mod modrinth;

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Add from modrinth
    #[command(alias = "mr")]
    Modrinth(modrinth::Args),
}

pub async fn run(app: App, args: Commands) -> Result<()> {
    match args {
        Commands::Modrinth(args) => modrinth::run(app, args).await?,
    }
    Ok(())
}

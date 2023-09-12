use anyhow::Result;

mod modrinth;

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Add from modrinth
    #[command(alias = "mr")]
    Modrinth(modrinth::Args),
}

pub async fn run(args: Commands) -> Result<()> {
    match args {
        Commands::Modrinth(args) => modrinth::run(args).await?,
    }
    Ok(())
}

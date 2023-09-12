use anyhow::Result;

mod unpack;

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(visible_alias = "unzip")]
    Unpack(unpack::Args),
}

pub async fn run(commands: Commands) -> Result<()> {
    match commands {
        Commands::Unpack(args) => unpack::run(args).await,
    }
}

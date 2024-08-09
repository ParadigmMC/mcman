use std::sync::Arc;

use anyhow::Result;

use crate::api::app::App;

pub mod mrpack;
pub mod packwiz;

#[derive(clap::Subcommand)]
pub enum Commands {
    MRPack(mrpack::Args),
    Packwiz(packwiz::Args),
}

pub async fn run(app: Arc<App>, args: Commands) -> Result<()> {
    match args {
        Commands::MRPack(args) => mrpack::run(app, args).await,
        Commands::Packwiz(args) => packwiz::run(app, args).await,
    }
}

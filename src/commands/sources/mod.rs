use std::sync::Arc;

use anyhow::Result;

use crate::api::app::App;

pub mod list;

#[derive(clap::Subcommand)]
pub enum Commands {
    List(list::Args),
}

pub async fn run(app: Arc<App>, args: Commands) -> Result<()> {
    match args {
        Commands::List(args) => list::run(app, args).await,
    }
}

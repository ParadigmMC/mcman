use anyhow::Result;

use crate::app::App;

mod unpack;

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(visible_alias = "unzip")]
    Unpack(unpack::Args),
}

pub fn run(app: &App, commands: Commands) -> Result<()> {
    match commands {
        Commands::Unpack(args) => unpack::run(app, args),
    }
}

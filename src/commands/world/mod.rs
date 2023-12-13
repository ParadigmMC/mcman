use anyhow::Result;

use crate::app::App;

mod pack;
mod unpack;

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(visible_alias = "zip")]
    Pack(pack::Args),
    #[command(visible_alias = "unzip")]
    Unpack(unpack::Args),
}

pub fn run(app: &mut App, commands: Commands) -> Result<()> {
    match commands {
        Commands::Pack(args) => pack::run(app, args),
        Commands::Unpack(args) => unpack::run(app, args),
    }
}

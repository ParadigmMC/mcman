use anyhow::Result;

use crate::app::App;

mod mrpack;
mod packwiz;

#[derive(clap::Subcommand)]
#[command(subcommand_required = true, arg_required_else_help = true)]
pub enum Commands {
    Mrpack(mrpack::Args),
    #[command(visible_alias = "pw")]
    Packwiz(packwiz::Args),
}

pub async fn run(mut app: App, commands: Commands) -> Result<()> {
    match commands {
        Commands::Mrpack(args) => mrpack::run(app, args).await,
        Commands::Packwiz(args) => packwiz::run(app, args).await,
    }
}

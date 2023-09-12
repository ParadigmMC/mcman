use anyhow::Result;

mod mrpack;
mod packwiz;

#[derive(clap::Subcommand)]
#[command(subcommand_required = true, arg_required_else_help = true)]
pub enum Commands {
    Mrpack(mrpack::Args),
    #[command(visible_alias = "pw")]
    Packwiz(packwiz::Args),
}

pub async fn run(commands: Commands) -> Result<()> {
    match commands {
        Commands::Mrpack(args) => mrpack::run(args).await,
        Commands::Packwiz(args) => packwiz::run(args).await,
    }
}

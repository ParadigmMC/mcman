use anyhow::Result;
use api::app::App;
use clap::Parser;

mod api;
mod commands;

#[derive(clap::Parser)]
#[clap(name = "mcman", version)]
#[command(author = "ParadigmMC", color = clap::ColorChoice::Always)]
#[command(about = "Powerful Minecraft Server Manager CLI")]
#[command(after_help = "To start building servers, try 'mcman init'")]
#[command(subcommand_required = true, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Init(commands::init::Args),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let app = App::new()?;

    match args.command {
        Commands::Init(args) => commands::init::run(app, args).await,
    }
}

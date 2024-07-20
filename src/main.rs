use std::sync::Arc;

use anyhow::Result;
use api::{app::App, utils::logger::init_logger};
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
    #[command(subcommand)]
    Sources(commands::sources::Commands),
    Build(commands::build::Args),
    #[command(subcommand)]
    Java(commands::java::Commands),
    #[command(alias = "md", subcommand)]
    Markdown(commands::markdown::Commands),
    Migrate(commands::migrate::Args),
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();
    let args = Cli::parse();
    let app = Arc::new(App::new()?);

    match args.command {
        Commands::Init(args) => commands::init::run(app, args).await,
        Commands::Sources(args) => commands::sources::run(app, args).await,
        Commands::Build(args) => commands::build::run(app, args).await,
        Commands::Java(args) => commands::java::run(app, args).await,
        Commands::Markdown(args) => commands::markdown::run(app, args).await,
        Commands::Migrate(args) => commands::migrate::run(app, args).await,
    }
}

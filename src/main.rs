use std::{path::Path, sync::Arc};

use anyhow::Result;
use api::{app::App, models::{packwiz::{PackwizPack, PACK_TOML}, server::{Server, SERVER_TOML}, ModpackSource, ModpackType, Source, SourceType}, utils::{logger::init_logger, toml::try_find_toml_upwards}};
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

    #[arg(global = true, long)]
    src: Vec<SourceType>,
}

#[derive(clap::Subcommand)]
enum Commands {
    Init(commands::init::Args),
    #[command(subcommand)]
    Sources(commands::sources::Commands),
    Build(commands::build::BuildArgs),
    Run(commands::run::RunArgs),
    #[command(subcommand)]
    Java(commands::java::Commands),
    #[command(alias = "md", subcommand)]
    Markdown(commands::markdown::Commands),
    #[command(subcommand)]
    Export(commands::export::Commands),
    Migrate(commands::migrate::Args),
    #[command(alias = "ws")]
    WebSocket(commands::websocket::Args),
    Update(commands::update::Args),
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    let args = Cli::parse();
    let app = Arc::new(App::new()?);

    if let Err(e) = app.try_read_files().await {
        println!("Error while reading files: {e:?}");
    }

    if !args.src.is_empty() {
        let mut wg = app.server.write().await;
        let (_, server) = wg.get_or_insert_with(|| (
            std::env::current_dir().unwrap().join(SERVER_TOML),
            Server::default()
        ));
        for source_type in args.src {
            server.sources.push(Source {
                source_type
            });
        }
    }

    if let Ok(Some((base, _))) = try_find_toml_upwards::<PackwizPack>(PACK_TOML) {
        let mut wg = app.server.write().await;
        // if no server.toml etc and is inside a packwiz folder
        if wg.is_none() {
            let (_, server) = wg.get_or_insert_with(|| (
                base.into(),
                Server::default()
            ));
            server.sources.push(Source {
                source_type: SourceType::Modpack {
                    modpack_source: ModpackSource::Local { path: String::from(PACK_TOML), can_update: false },
                    modpack_type: ModpackType::Packwiz,
                }
            });
        }
    }

    match args.command {
        Commands::Init(args) => commands::init::run(app, args).await,
        Commands::Sources(args) => commands::sources::run(app, args).await,
        Commands::Build(args) => commands::build::run(app, args).await,
        Commands::Run(args) => commands::run::run(app, args).await,
        Commands::Java(args) => commands::java::run(app, args).await,
        Commands::Markdown(args) => commands::markdown::run(app, args).await,
        Commands::Migrate(args) => commands::migrate::run(app, args).await,
        Commands::WebSocket(args) => commands::websocket::run(app, args).await,
        Commands::Update(args) => commands::update::run(app, args).await,
        Commands::Export(args) => commands::export::run(app, args).await,
    }
}

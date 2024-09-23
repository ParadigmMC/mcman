use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;

use crate::api::{
    app::App,
    models::{
        network::{Network, NETWORK_TOML},
        server::{Server, SERVER_TOML},
    },
    utils::toml::write_toml,
};

#[derive(clap::Args)]
pub struct Args {
    /// The name of the server
    #[arg(long)]
    name: Option<String>,

    /// Folder to initialize in
    #[arg(long, default_value = ".")]
    dir: String,
}

pub async fn run(app: Arc<App>, args: Args) -> Result<()> {
    init_server(app, args).await?;

    Ok(())
}

pub async fn init_server(app: Arc<App>, args: Args) -> Result<()> {
    cliclack::intro("Creating a new server...")?;

    let dir = PathBuf::from(args.dir);
    let path = dir.join(SERVER_TOML);

    if let Some((_, server)) = &*app.server.read().await {
        let con = cliclack::confirm(format!(
            "Server with name '{}' found! Continue?",
            server.name
        ))
        .initial_value(false)
        .interact()?;

        if con {
            cliclack::note("Warning", "Current server might get overwritten")?;
        } else {
            cliclack::outro_cancel("Cancelled")?;
            return Ok(());
        }
    }

    let name: String = args
        .name
        .map(Ok)
        .unwrap_or_else(|| cliclack::input("Name of the server?").interact())?;

    let mut server = Server {
        name,
        ..Default::default()
    };

    {
        write_toml(&dir, SERVER_TOML, &server)?;

        let mut wg = app.server.write().await;
        *wg = Some((path, server));
    }

    cliclack::outro("Saved!")?;

    Ok(())
}

pub async fn action_init_network(app: Arc<App>, args: Args) -> Result<()> {
    cliclack::intro("initializing network")?;

    let dir = PathBuf::from(args.dir);
    let path = dir.join(NETWORK_TOML);

    let name: String = args
        .name
        .map(Ok)
        .unwrap_or_else(|| cliclack::input("Name of the network?").interact())?;

    let mut nw = Network {
        name,
        ..Default::default()
    };

    {
        write_toml(&dir, NETWORK_TOML, &nw)?;

        let mut wg = app.network.write().await;
        *wg = Some((path, nw));
    }

    cliclack::outro("Saved!")?;

    Ok(())
}

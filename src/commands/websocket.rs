use std::sync::Arc;

use anyhow::{Context, Result};

use crate::api::{app::App, ws::WebsocketServer};

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, default_value = "0.0.0.0:6969")]
    addr: String,
}

pub async fn run(app: Arc<App>, args: Args) -> Result<()> {
    let ws = WebsocketServer::new(app);

    ws.start(&args.addr).await.context("Running WebSocket Server")?;

    Ok(())
}

use std::{path::Path, sync::Arc};

use anyhow::Result;

use crate::api::app::App;

#[derive(clap::Args)]
pub struct Args {
    /// The name of the server
    #[arg(long)]
    name: Option<String>,
}

pub async fn run(app: Arc<App>, args: Args) -> Result<()> {
    app.action_init_server()
        .await?;

    Ok(())
}

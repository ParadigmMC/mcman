use std::path::Path;

use anyhow::Result;

use crate::api::app::App;

#[derive(clap::Args)]
pub struct Args {
    /// The name of the server
    #[arg(long)]
    name: Option<String>,
}

pub async fn run(mut app: App, args: Args) -> Result<()> {
    app.action_install_addons(Path::new("./output/server"))
        .await?;

    Ok(())
}

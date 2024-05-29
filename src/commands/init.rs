use anyhow::Result;

use crate::api::app::App;

#[derive(clap::Args)]
pub struct Args {
    /// The name of the server
    #[arg(long)]
    name: Option<String>,
}



pub async fn run(app: App, args: Args) -> Result<()> {
    Ok(())
}

use std::sync::Arc;

use anyhow::Result;

use crate::api::app::App;

#[derive(clap::Args)]
pub struct Args {
    #[clap(default_value = "./metadata.json")]
    filename: String,
}

pub async fn run(app: Arc<App>, args: Args) -> Result<()> {
    

    Ok(())
}
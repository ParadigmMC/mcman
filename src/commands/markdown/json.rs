use std::{path::PathBuf, sync::Arc};

use anyhow::Result;

use crate::api::app::App;

#[derive(clap::Args)]
pub struct Args {
    #[clap(short, long, default_value = "./metadata.json")]
    output: PathBuf,

    #[clap(short, long)]
    pretty: bool,
}

pub async fn run(app: Arc<App>, args: Args) -> Result<()> {
    let metadata = app.get_metadata().await?;

    std::fs::write(args.output, if args.pretty {
        serde_json::to_string_pretty(&metadata)
    } else {
        serde_json::to_string(&metadata)
    }?)?;

    Ok(())
}

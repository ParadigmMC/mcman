use std::{path::Path, sync::Arc};

use anyhow::Result;

use crate::api::app::App;

#[derive(clap::Args)]
pub struct Args {
    
}

pub async fn run(app: Arc<App>, args: Args) -> Result<()> {
    let base = Path::new("./output/server");
    
    app.action_install_jar(&base)
        .await?;

    app.action_install_addons(&base)
        .await?;

    Ok(())
}

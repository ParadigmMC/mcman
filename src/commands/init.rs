use anyhow::Result;

use crate::api::app::App;

#[derive(clap::Args)]
pub struct Args {
    /// The name of the server
    #[arg(long)]
    name: Option<String>,
}



pub async fn run(mut app: App, args: Args) -> Result<()> {
    let addons = app.collect_addons().await?;

    println!("{addons:#?}");

    Ok(())
}

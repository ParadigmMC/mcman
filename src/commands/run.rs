use std::{path::PathBuf, sync::Arc};

use anyhow::{Context, Result};

use crate::api::{app::App, tools::java::{get_java_installation_for, JavaProcess}, ws::WebsocketServer};

#[derive(clap::Args)]
pub struct RunArgs {
    #[command(flatten)]
    pub build_args: super::build::BuildArgs,
}

pub async fn run(app: Arc<App>, args: RunArgs) -> Result<()> {
    let base = args.build_args.get_base_dir(&app).await?;

    super::build::run(app.clone(), args.build_args).await?;
    
    let rg = app.server.read().await;
    let (_, server) = rg.as_ref().unwrap();
    let java = server.get_java().await;
    let args = server.get_arguments();
    drop(rg);

    log::info!("Starting process...");

    let mut process = JavaProcess::new(&base, java, args)?;

    process.lines(|line| {
        println!("| {line}");
    });

    let e = process.wait().await?;

    println!("{e:#?}");

    Ok(())
}


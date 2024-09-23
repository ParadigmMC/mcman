use std::sync::Arc;

use anyhow::Result;

use crate::api::{app::App, tools::java::JavaProcess};

#[derive(clap::Args)]
pub struct RunArgs {
    #[command(flatten)]
    pub build_args: super::build::BuildArgs,
}

pub async fn run(app: Arc<App>, args: RunArgs) -> Result<()> {
    let base = args.build_args.get_base_dir(&app).await?;

    super::build::run(app.clone(), args.build_args).await?;

    let (java, args) = if let Some((_, server)) = &*app.server.read().await {
        (server.get_java().await, server.get_arguments())
    } else {
        unreachable!();
    };

    log::info!("Starting process...");

    let mut process = JavaProcess::new(&base, java, args)?;

    process.lines(|line| {
        println!("| {line}");
    });

    let exit_status = tokio::select! {
        _ = tokio::signal::ctrl_c() => None,
        Ok(e) = process.wait() => Some(e),
    };

    if let Some(e) = exit_status {
        println!("{e:#?}");
    } else {
        process.kill().await?;
        println!("Killed process");
    }

    Ok(())
}

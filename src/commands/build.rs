use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::{
    core::BuildContext,
    model::Lockfile, app::App,
};

#[derive(clap::Args)]
pub struct BuildArgs {
    /// The output directory for the server
    #[arg(short, long, value_name = "file")]
    output: Option<PathBuf>,
    /// Skip some stages
    #[arg(short, long, value_name = "stages")]
    skip: Vec<String>,
    #[arg(long)]
    /// Don't skip downloading already downloaded jars
    force: bool,
}

impl<'a> BuildArgs {
    pub fn create_build_context(&self, app: &'a App) -> Result<BuildContext<'a>> {
        let default_output = app.server.path.join("server");
        let output_dir = self.output.clone().unwrap_or(default_output);

        let force = self.force;
        let skip_stages = self.skip.clone();

        std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

        Ok(BuildContext {
            app,
            force,
            skip_stages,
            output_dir,
            lockfile: Lockfile::default(),
            new_lockfile: Lockfile::default(),
            server_process: None,
        })
    }
}

pub async fn run(app: App, args: BuildArgs) -> Result<()> {
    let mut ctx = args.create_build_context(&app)?;

    ctx.build_all().await?;

    Ok(())
}

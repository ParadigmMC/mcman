use anyhow::{anyhow, Context, Result};

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    build_args: crate::commands::build::Args,
    /// Test the server (stops it when it ends startup)
    #[arg(long)]
    test: bool,
}

pub async fn run(args: Args) -> Result<()> {
    let mut ctx = super::build::run(args.build_args).await?;

    let test_mode = args.test;

    ctx.run(test_mode).context("Starting child process")?;
    let status = ctx.pipe_child_process(test_mode).await?;

    match status.code() {
        Some(i) if i > 0 => Err(anyhow!("java exited with code {i}")),
        _ => Ok(()),
    }
}

use anyhow::{anyhow, Context, Result};
use clap::{arg, ArgMatches, Command};

pub fn cli() -> Command {
    super::build::cli()
        .name("run")
        .arg(arg!(--test "Test the server (stops it when it ends startup)"))
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let mut ctx = super::build::run(matches).await?;

    let test_mode = matches.get_flag("test");

    ctx.run(test_mode).context("Starting child process")?;
    let status = ctx.pipe_child_process(test_mode).await?;

    match status.code() {
        Some(i) if i > 0 => Err(anyhow!("java exited with code {i}")),
        _ => Ok(()),
    }
}

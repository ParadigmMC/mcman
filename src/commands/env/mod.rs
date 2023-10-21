use anyhow::Result;

use crate::app::App;

mod docker;
mod gitignore;

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Modify the gitignore
    Gitignore,
    /// Write the default Dockerfile and .dockerignore
    Docker,
}

pub fn run(app: App, commands: Commands) -> Result<()> {
    match commands {
        Commands::Gitignore => gitignore::run(app),
        Commands::Docker => docker::run(app),
    }
}

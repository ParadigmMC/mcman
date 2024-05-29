use anyhow::Result;

use crate::app::App;

mod docker;
mod gitignore;
mod workflow_packwiz;
mod workflow_test;

#[derive(clap::Subcommand, Clone, Copy)]
pub enum Commands {
    /// Modify the gitignore
    Gitignore,
    /// Write the default Dockerfile and .dockerignore
    Docker,
    /// github workflow: test the server
    Test,
    /// github workflow: export packwiz automatically
    Packwiz,
}

pub fn run(app: &App, commands: Commands) -> Result<()> {
    match commands {
        Commands::Gitignore => gitignore::run(app),
        Commands::Docker => docker::run(app),
        Commands::Packwiz => workflow_packwiz::run(app),
        Commands::Test => workflow_test::run(app),
    }
}

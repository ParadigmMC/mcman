use anyhow::Result;

mod docker;
mod gitignore;

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Modify the gitignore
    Gitignore,
    /// Write the default Dockerfile and .dockerignore
    Docker,
}

pub fn run(commands: Commands) -> Result<()> {
    match commands {
        Commands::Gitignore => gitignore::run(),
        Commands::Docker => docker::run(),
    }
}

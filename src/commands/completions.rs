use anyhow::Result;
use clap::{Args, Command, CommandFactory};
use clap_complete::{generate, Generator, Shell};
use std::io;

use crate::Cli;

/// Completion arguments.
#[derive(Args)]
pub struct CompletionArgs {
    /// Generate completions for the specified shell.
    pub shell: Shell,
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

pub fn run(args: &CompletionArgs) -> Result<()> {
    print_completions(args.shell, &mut Cli::command());
    std::process::exit(0);
}

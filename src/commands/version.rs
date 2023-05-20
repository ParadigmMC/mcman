use clap::{ArgMatches, Command};

pub fn cli() -> Command {
    Command::new("version").about("Show version information")
}

pub fn run(_matches: &ArgMatches) {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

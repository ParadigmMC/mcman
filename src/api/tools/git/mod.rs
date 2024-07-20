use std::{ffi::OsStr, process::Command};

use anyhow::Result;

pub const GIT: &str = "git";

pub fn git_command<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> Result<String> {
    Ok(String::from_utf8_lossy(Command::new(GIT)
        .args(args)
        .output()?
        .stdout
        .as_slice()
    ).into_owned())
}

pub fn git_check() -> Option<String> {
    git_command(["--version"]).ok().map(|s| s.replacen("git version ", "", 1))
}



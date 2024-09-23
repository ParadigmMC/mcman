use std::{ffi::OsStr, process::Command, sync::LazyLock};

use anyhow::{anyhow, Result};

pub const GIT: &str = "git";

static GIT_VERSION: LazyLock<Option<String>> = LazyLock::new(version_check);

pub fn require_git() -> Result<()> {
    GIT_VERSION
        .as_ref()
        .map(|_| ())
        .ok_or(anyhow!("Couldn't invoke git"))
}

pub fn git_command<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> Result<String> {
    Ok(
        String::from_utf8_lossy(Command::new(GIT).args(args).output()?.stdout.as_slice())
            .into_owned(),
    )
}

pub fn version_check() -> Option<String> {
    git_command(["--version"])
        .ok()
        .map(|s| s.trim().replacen("git version ", "", 1))
}

pub fn is_dirty() -> Result<bool> {
    require_git()?;
    Ok(!git_command(["status", "--porcelain"])?.is_empty())
}

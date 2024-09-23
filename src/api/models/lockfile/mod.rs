use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

mod bootstrapped_file;
pub use bootstrapped_file::*;

use super::Addon;

pub const LOCKFILE: &str = ".mcman.lock";

#[derive(Debug, Serialize, Deserialize)]
#[derive(Default)]
pub struct Lockfile {
    pub vars: HashMap<String, String>,
    pub addons: Vec<Addon>,
    pub bootstrapped_files: HashMap<PathBuf, BootstrappedFile>,
}


pub enum LockfileMessage {
    BootstrapFile(PathBuf, BootstrappedFile),
}

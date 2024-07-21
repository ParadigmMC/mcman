use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};

mod bootstrapped_file;
pub use bootstrapped_file::*;

use super::Addon;

pub const LOCKFILE: &str = ".mcman.lock";

#[derive(Debug, Serialize, Deserialize)]
pub struct Lockfile {
    pub vars: HashMap<String, String>,
    pub addons: Vec<Addon>,
    pub bootstrapped_files: HashMap<PathBuf, BootstrappedFile>,
}

impl Default for Lockfile {
    fn default() -> Self {
        Self {
            addons: vec![],
            bootstrapped_files: HashMap::new(),
            vars: HashMap::new(),
        }
    }
}

pub enum LockfileMessage {
    BootstrapFile(PathBuf, BootstrappedFile),
}

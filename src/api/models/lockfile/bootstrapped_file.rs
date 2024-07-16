use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BootstrappedFile {
    pub date: SystemTime,
    pub hash: Option<String>,
    pub vars: Vec<String>,
}



use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::models::{Environment, Step};

use super::AddonType;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
pub enum AddonTarget {
    Plugin,
    Mod,
    Custom(String),
}

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
pub struct Addon {
    pub environment: Option<Environment>,
    pub addon_type: AddonType,
    pub target: AddonTarget,
}

impl Addon {
    async fn resolve_steps(&self) -> Result<Vec<Step>> {
        Ok(vec![])
    }
}

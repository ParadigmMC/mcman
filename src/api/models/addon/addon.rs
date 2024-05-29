use anyhow::Result;

use crate::api::models::{Environment, Step};

use super::AddonType;

pub enum AddonTarget {
    Plugin,
    Mod,
    Custom(String),
}

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

use anyhow::Result;

use crate::app::AddonType;

use super::{Environment, Step};

pub enum AddonTarget {
    Plugin,
    Mod,
}

pub struct Addon {
    pub environment: Option<Environment>,
    pub addon_type: AddonType,
}

impl Addon {
    async fn resolve_steps(&self) -> Result<Vec<Step>> {
        Ok(vec![])
    }
}

use std::{path::Path, sync::Arc};

use anyhow::Result;

use crate::api::app::App;

pub mod addons;
pub mod server_jar;
pub mod worlds;
pub mod bootstrap;

impl App {
    pub async fn action_build(self: Arc<Self>, base: &Path) -> Result<()> {
        self.action_install_jar(base).await?;
        self.clone().action_install_addons(base).await?;
        self.clone().action_bootstrap(base).await?;
        self.action_generate_scripts(base).await?;

        Ok(())
    }
}

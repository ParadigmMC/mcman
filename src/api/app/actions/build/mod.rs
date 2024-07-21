use std::{path::Path, sync::Arc};

use anyhow::Result;

use crate::api::app::App;

pub mod addons;
pub mod server_jar;
pub mod worlds;
pub mod bootstrap;

impl App {
    /// Builds the entire server
    pub async fn action_build(self: Arc<Self>, base: &Path) -> Result<()> {
        self.read_existing_lockfile(base).await?;

        self.action_install_jar(base).await?;
        self.clone().action_install_addons(base).await?;
        self.clone().action_bootstrap(base).await?;
        self.action_generate_scripts(base).await?;

        self.write_lockfile(base).await?;

        Ok(())
    }
}

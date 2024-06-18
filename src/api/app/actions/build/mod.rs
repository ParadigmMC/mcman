use std::path::Path;

use anyhow::Result;

use crate::api::{app::App, models::Addon};

impl App {
    pub async fn action_install_jar(&self) -> Result<()> {
        Ok(())
    }

    pub async fn action_install_addon(&self, base: &Path, addon: &Addon) -> Result<()> {
        let steps = addon.resolve_steps(&self).await?;
        let dir = base.join(addon.target.as_str());
        self.execute_steps(&dir, &steps).await?;
        Ok(())
    }

    pub async fn action_install_addons(&self, base: &Path) -> Result<()> {
        let addons = self.collect_addons().await?;

        for addon in &addons {
            self.action_install_addon(base, addon).await?;
        }

        Ok(())
    }
}

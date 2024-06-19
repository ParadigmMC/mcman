use std::path::Path;

use anyhow::Result;

use crate::api::{app::App, models::{Addon, Environment}};

impl App {
    pub async fn action_install_jar(&self, base: &Path) -> Result<()> {
        if let Some(server) = &self.server {
            let server = server.read().await;

            if let Some(jar) = &server.jar {
                let steps = jar.resolve_steps(&self, Environment::Server).await?;

                self.execute_steps(base, &steps).await?;
            }
        }

        Ok(())
    }

    pub async fn action_install_addons(&self, base: &Path) -> Result<()> {
        let addons = self.collect_addons().await?;

        for addon in &addons {
            self.action_install_addon(base, addon).await?;
        }

        Ok(())
    }

    pub async fn action_install_addon(&self, base: &Path, addon: &Addon) -> Result<()> {
        let steps = addon.resolve_steps(&self).await?;
        let dir = base.join(addon.target.as_str());
        self.execute_steps(&dir, &steps).await?;
        Ok(())
    }
}

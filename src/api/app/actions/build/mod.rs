use std::{path::Path, sync::Arc};

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

    pub async fn action_install_addons(self: Arc<Self>, base: &Path) -> Result<()> {
        let addons = self.collect_addons().await?;
        let base = Arc::new(base.to_owned());

        let mut handles = vec![];

        for addon in addons {
            let app = self.clone();
            let base = base.clone();
            handles.push(tokio::spawn(async move {
                app.action_install_addon(&base, &addon).await
            }));
        }

        for handle in handles {
            handle.await??;
        }

        Ok(())
    }

    pub async fn action_install_addon(self: Arc<Self>, base: &Path, addon: &Addon) -> Result<()> {
        let steps = addon.resolve_steps(&self).await?;
        let dir = base.join(addon.target.as_str());
        self.execute_steps(&dir, &steps).await?;
        Ok(())
    }
}

use std::{path::Path, sync::Arc};

use anyhow::Result;

use crate::api::{app::App, models::{lockfile::{Lockfile, LOCKFILE}, Addon}};

impl App {
    pub async fn reset_lockfile(&self) -> Result<()> {
        let mut new_lockfile = self.new_lockfile.write().await;
        *new_lockfile = Lockfile::default();

        Ok(())
    }

    pub async fn read_existing_lockfile(&self, base: &Path) -> Result<()> {
        let path = base.join(LOCKFILE);
        
        if path.exists() {
            let mut existing_lockfile = self.existing_lockfile.write().await;
            *existing_lockfile = Some(serde_json::from_str::<Lockfile>(&tokio::fs::read_to_string(base.join(LOCKFILE)).await?)?);
        }

        Ok(())
    }

    pub async fn write_lockfile(&self, base: &Path) -> Result<()> {
        let lockfile = self.new_lockfile.read().await;

        tokio::fs::write(base.join(LOCKFILE), serde_json::to_vec(&*lockfile)?).await?;

        Ok(())
    }

    pub async fn add_addon_to_lockfile(self: Arc<Self>, addon: Addon) {
        println!("Added Addon to lockfile");
        self.new_lockfile.write().await.addons.push(addon);
    }
}

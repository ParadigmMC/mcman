use std::{path::Path, sync::Arc};
use futures::stream::{self, StreamExt, TryStreamExt};

use anyhow::Result;

use crate::api::{app::App, models::{Addon, Environment}};

pub mod addons;
pub mod server_jar;

impl App {
    pub async fn action_build(self: Arc<Self>, base: &Path) -> Result<()> {
        self.action_install_jar(base).await?;
        self.clone().action_install_addons(base).await?;

        self.action_generate_script().await?;

        Ok(())
    }
}

use std::path::Path;

use anyhow::Result;

use crate::api::{app::App, models::Environment};

impl App {
    /// Installs the server jar according to [`crate::api::models::server::Server::get_jar`]
    pub async fn action_install_jar(&self, base: &Path) -> Result<()> {
        if let Some((_, server)) = &*self.server.read().await {
            println!("Installing server jar");

            let jar = server.get_jar(self).await?;

            let steps = jar.resolve_steps(self, Environment::Server).await?;

            self.execute_steps(base, &steps).await?;
        }

        Ok(())
    }
}

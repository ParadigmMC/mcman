use std::path::Path;

use anyhow::Result;

use crate::api::{app::App, models::Environment};

impl App {
    /// Installs the server jar according to [`crate::api::models::server::Server::jar`]
    pub async fn action_install_jar(&self, base: &Path) -> Result<()> {
        if let Some(jar) = self.server.read().await.as_ref().map(|(_, server)| {
            server.jar.clone()
        }).flatten() {
            println!("Installing server jar");

            let steps = jar.resolve_steps(&self, Environment::Server).await?;

            self.execute_steps(base, &steps).await?;
        }

        Ok(())
    }
}

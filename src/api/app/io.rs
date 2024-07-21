use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;

use crate::api::{models::{lockfile::Lockfile, network::{Network, NETWORK_TOML}, server::{Server, SERVER_TOML}}, utils::toml::{try_find_toml_upwards, write_toml}};

use super::App;

impl App {
    pub fn try_read_files(&mut self) -> Result<()> {
        let server = try_find_toml_upwards::<Server>(SERVER_TOML)?;
        let network = try_find_toml_upwards::<Network>(NETWORK_TOML)?;

        self.server = Arc::new(RwLock::new(server));
        self.network = Arc::new(RwLock::new(network));

        Ok(())
    }

    pub async fn save_changes(&self) -> Result<()> {
        if let Some((path, server)) = &*self.server.read().await {
            write_toml(path.parent().unwrap(), SERVER_TOML, &server)?;
        }

        if let Some((path, network)) = &*self.network.read().await {
            write_toml(path.parent().unwrap(), NETWORK_TOML, &network)?;
        }

        Ok(())
    }
}

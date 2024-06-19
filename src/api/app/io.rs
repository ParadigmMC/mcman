use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;

use crate::api::{models::{network::{Network, NETWORK_TOML}, server::{Server, SERVER_TOML}}, utils::{try_find_toml_upwards, write_toml}};

use super::App;

impl App {
    pub fn try_read_files(&mut self) -> Result<()> {
        let (server_path, server) = try_find_toml_upwards::<Server>(SERVER_TOML)?.unzip();
        let (network_path, network) = try_find_toml_upwards::<Network>(NETWORK_TOML)?.unzip();

        self.server_path = server_path;
        self.server = server.map(|s| Arc::new(RwLock::new(s)));
        self.network_path = network_path;
        self.network = network.map(|nw| Arc::new(RwLock::new(nw)));

        Ok(())
    }

    pub async fn save_changes(&self) -> Result<()> {
        if let Some((path, server)) = self.server_path.as_ref().zip(self.server.as_ref()) {
            let server = server.read().await;
            write_toml(&path, SERVER_TOML, &*server)?;
        }

        if let Some((path, network)) = self.network_path.as_ref().zip(self.network.as_ref()) {
            let network = network.read().await;
            write_toml(&path, NETWORK_TOML, &*network)?;
        }

        Ok(())
    }
}

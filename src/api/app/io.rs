use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;

use crate::api::{
    models::{
        lockfile::Lockfile,
        network::{Network, NETWORK_TOML},
        server::{Server, SERVER_TOML},
    },
    utils::toml::{try_find_toml_upwards, write_toml},
};

use super::App;

impl App {
    pub async fn try_read_files(&self) -> Result<()> {
        let server = try_find_toml_upwards::<Server>(SERVER_TOML)?;
        let network = try_find_toml_upwards::<Network>(NETWORK_TOML)?;

        let mut swg = self.server.write().await;
        *swg = server;
        let mut nwg = self.network.write().await;
        *nwg = network;

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

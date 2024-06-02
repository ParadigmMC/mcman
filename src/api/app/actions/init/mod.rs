use std::sync::Arc;

use anyhow::Result;
use cliclack::{input, intro};
use tokio::sync::RwLock;

use crate::api::{app::App, models::{network::Network, server::Server}};

impl App {
    pub async fn init_server(&mut self) -> Result<()> {
        intro("initializing server")?;

        let name: String = input("Name of the server?")
            .interact()?;

        let mut server = Server {
            name,
            port: None,
            sources: vec![],
        };

        self.server = Some(Arc::new(RwLock::new(server)));

        Ok(())
    }

    pub async fn init_network(&mut self) -> Result<()> {
        intro("initializing network")?;

        let name: String = input("Name of the network?")
            .interact()?;

        let mut nw = Network {
            name,
        };

        self.network = Some(Arc::new(RwLock::new(nw)));

        Ok(())
    }
}

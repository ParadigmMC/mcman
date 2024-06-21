use std::path::PathBuf;

use anyhow::Result;

use crate::api::{
    app::App,
    models::{network::Network, server::{Server, SERVER_TOML}},
};

impl App {
    pub async fn action_init_server(&self) -> Result<()> {
        cliclack::intro("Creating a new server...")?;

        let path = PathBuf::from(".").join(SERVER_TOML);

        if let Some((_, server)) = &*self.server.read().await {
            let con = cliclack::confirm(format!("Server with name '{}' found! Continue?", server.name))
                .initial_value(false)
                .interact()?;

            if con {
                cliclack::note("Warning", "Current server might get overwritten")?;
            } else {
                cliclack::outro_cancel("Cancelled")?;
                return Ok(());
            }
        }

        let name: String = cliclack::input("Name of the server?").interact()?;

        let mut server = Server {
            name,
            ..Default::default()
        };

        {
            let mut wg = self.server.write().await;
            *wg = Some((path, server));
        }

        cliclack::outro("Saved!")?;

        Ok(())
    }

    pub async fn action_init_network(&self) -> Result<()> {
        cliclack::intro("initializing network")?;

        let name: String = cliclack::input("Name of the network?").interact()?;

        let mut nw = Network { name };

        //self.network = Some(Arc::new(RwLock::new(nw)));

        Ok(())
    }
}

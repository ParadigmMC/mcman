use anyhow::Result;

use crate::api::app::App;

impl App {
    pub async fn action_bootstrap(&self) -> Result<()> {
        Ok(())
    }
}

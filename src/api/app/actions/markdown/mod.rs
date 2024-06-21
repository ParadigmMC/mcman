use std::sync::Arc;

use anyhow::Result;

use crate::api::app::App;

impl App {
    pub async fn render_markdown(self: Arc<Self>) -> Result<()> {
        let addons = self.collect_addons().await?;

        

        Ok(())
    }
}

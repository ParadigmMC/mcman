use anyhow::Result;

use super::App;

impl App {
    pub async fn refresh_markdown(&self) -> Result<()> {
        if self.server.markdown.auto_update {
            self.markdown().update_files().await
        } else {
            Ok(())
        }
    }
}

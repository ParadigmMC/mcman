use std::sync::Arc;

use anyhow::Result;

use crate::api::{app::App, models::{markdown::{MarkdownOptions, MarkdownOutput}, metadata::{AddonMetadata, AddonMetadataSource}}};

impl App {
    pub async fn get_markdown_options(&self) -> Option<MarkdownOptions> {
        if let Some((_, server)) = &*self.server.read().await {
            Some(server.markdown.clone())
        } else {
            None
        }
    }

    pub async fn get_all_metadata(self: Arc<Self>) -> Result<Vec<AddonMetadata>> {
        let addons = self.collect_addons().await?;

        let mut metadata = vec![];

        for addon in &addons {
            // TODO
            if let Ok(m) = addon.resolve_metadata(&self).await {
                metadata.push(m);
            }
        }

        Ok(metadata)
    }

    pub async fn render_markdown_with(&self, metadata: Vec<AddonMetadata>) -> Result<String> {
        let markdown_options = self.get_markdown_options().await.unwrap_or_default();

        let table = markdown_options.render(metadata, markdown_options.output_type);

        Ok(table.render(markdown_options.output_type))
    }

    pub async fn render_markdown_and_save(self: Arc<Self>) -> Result<()> {
        let metadata = self.clone().get_all_metadata().await?;
        let content = self.render_markdown_with(metadata).await?;
        let options = self.get_markdown_options().await.unwrap_or_default();

        // TODO

        Ok(())
    }
}

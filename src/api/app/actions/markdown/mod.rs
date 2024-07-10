use std::{collections::HashMap, sync::Arc};

use anyhow::Result;

use crate::api::{app::App, models::{markdown::MarkdownOptions, metadata::{AddonMetadata, MetadataContainer}}};

impl App {
    pub async fn get_markdown_options(&self) -> Option<MarkdownOptions> {
        if let Some((_, server)) = &*self.server.read().await {
            Some(server.markdown.clone())
        } else {
            None
        }
    }

    pub async fn get_all_addon_metadata(self: Arc<Self>) -> Result<Vec<AddonMetadata>> {
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

    pub async fn get_metadata(self: Arc<Self>) -> Result<MetadataContainer> {
        let addons = self.get_all_addon_metadata().await?;

        Ok(MetadataContainer {
            addons
        })
    }

    pub async fn render_metadata(&self, metadata: MetadataContainer) -> Result<HashMap<String, String>> {
        let markdown_options = self.get_markdown_options().await.unwrap_or_default();

        let mut map = HashMap::new();

        map.insert(String::from("addons"), markdown_options.render_addons(metadata.addons));

        Ok(map)
    }

    pub async fn render_addon_metadata(&self, metadata: Vec<AddonMetadata>) -> Result<String> {
        let markdown_options = self.get_markdown_options().await.unwrap_or_default();

        let table = markdown_options.table_addons(metadata, markdown_options.output_type);

        Ok(table.render(markdown_options.output_type))
    }

    pub async fn render_markdown_and_save(self: Arc<Self>) -> Result<()> {
        let metadata = self.clone().get_metadata().await?;
        let rendered = self.render_metadata(metadata).await?;

        // TODO

        Ok(())
    }
}

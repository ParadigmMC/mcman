use std::path::PathBuf;

use anyhow::Result;

use crate::api::models::{Addon, Source};

use super::App;

impl App {
    pub async fn collect_sources(&self) -> Result<Vec<(PathBuf, Source)>> {
        let mut sources = vec![];

        if let Some((server_path, server)) = &*self.server.read().await {
            let server_path = server_path.parent().unwrap().to_path_buf();
            if let Some((network_path, network)) = &*self.network.read().await {
                let network_path = network_path.parent().unwrap().to_path_buf();

                if let Some(group) = network.groups.get("global") {
                    for source in &group.sources {
                        sources.push((network_path.clone(), source.clone()));
                    }
                }

                if let Some(entry) = network.servers.get(&server.name) {
                    for group_name in &entry.groups {
                        if let Some(group) = network.groups.get(group_name) {
                            for source in &group.sources {
                                sources.push((network_path.clone(), source.clone()));
                            }
                        }
                    }
                }
            }

            for source in &server.sources {
                sources.push((server_path.clone(), source.clone()));
            }
        }

        Ok(sources)
    }

    pub async fn collect_addons(&self) -> Result<Vec<Addon>> {
        let mut addons = vec![];

        for (relative_to, source) in self.collect_sources().await? {
            addons.extend_from_slice(&source.resolve_addons(self, &relative_to).await?);
        }

        Ok(addons)
    }

    pub async fn collect_bootstrap_paths(&self) -> Vec<PathBuf> {
        let mut list = vec![];

        if let Some((server_path, server)) = &*self.server.read().await {
            if let Some((network_path, network)) = &*self.network.read().await {
                // - network.toml
                // - groups/global/**
                list.push(
                    network_path
                        .parent()
                        .unwrap()
                        .join("groups")
                        .join("global")
                        .join("config"),
                );

                if let Some(entry) = network.servers.get(&server.name) {
                    for group in &entry.groups {
                        // - network.toml
                        // - groups/<name>/**
                        list.push(
                            network_path
                                .parent()
                                .unwrap()
                                .join("groups")
                                .join(group)
                                .join("config"),
                        );
                    }
                }
            }

            // - server.toml
            // - config/**
            list.push(server_path.parent().unwrap().join("config"));
        }

        list
    }
}

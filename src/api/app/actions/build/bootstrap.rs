use std::{collections::HashSet, path::Path, sync::Arc};

use anyhow::{Context, Result};
use futures::{StreamExt, TryStreamExt};
use walkdir::WalkDir;

use crate::api::app::App;

impl App {
    pub async fn action_bootstrap(self: Arc<Self>, base: &Path) -> Result<()> {
        let mut list = vec![];

        if let Some(path) = self.server.read().await.as_ref().map(|(path, _)| path.clone()) {
            list.push(path);
        }

        // TODO: use self.network

        for entry in list {
            self.clone().action_bootstrap_recursive(base, &entry).await?;
        }

        Ok(())
    }

    pub async fn action_bootstrap_recursive(self: Arc<Self>, base: &Path, from: &Path) -> Result<()> {
        let base = Arc::new(base);
        if !from.exists() {
            return Ok(());
        }

        const MAX_CONCURRENT_TASKS: usize = 20;

        futures::stream::iter(WalkDir::new(from)).map(|res| res.with_context(|| format!("Bootstrapping folder: {from:?}"))).try_for_each_concurrent(
            Some(MAX_CONCURRENT_TASKS),
            move |entry| {
                let app = self.clone();
                let base = base.clone();
                async move {
                    app.action_bootstrap_file(&base, entry.path())
                        .await
                        .with_context(|| format!("Bootstrapping file: {:?}", entry.path()))
                }
            }
        ).await?;

        Ok(())
    }

    pub async fn action_bootstrap_file(self: Arc<Self>, base: &Path, file: &Path) -> Result<()> {
        let lockfile_entry = self.existing_lockfile.read().await.as_ref().map(|lock| lock.bootstrapped_files.get(file)).flatten().cloned();

        

        Ok(())
    }
}

use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use futures::TryStreamExt;

use crate::api::{app::App, step::{FileMeta, StepResult}};

impl App {
    // if FileMeta has cache location,
    //   download to cache dir
    //   copy from cache
    // else
    //   download to destination dir
    pub(super) async fn execute_step_download(&self, dir: &Path, url: &str, metadata: &FileMeta) -> Result<StepResult> {
        let cache_destination = self.cache.loc(metadata.cache.as_ref());
        let output_destination = dir.join(&metadata.filename);

        let res = self.http_get(url).await?;

        let content_length = res.content_length();
        match (metadata.size, content_length) {
            (Some(a), Some(b)) if a != b => {
                bail!("Mismatched Content-Length! Expected {a}, recieved {b}");
            }
            _ => {}
        }

        let mut stream = res.bytes_stream();

        let mut hasher = metadata.get_hasher();

        let target_destination = cache_destination.as_ref().unwrap_or(&output_destination);
        tokio::fs::create_dir_all(target_destination.parent().ok_or(anyhow!("No parent"))?)
            .await?;

        let target_file = tokio::fs::File::create(&target_destination).await
            .with_context(|| format!("Creating file: {}", target_destination.display()))?;
        let mut target_writer = tokio::io::BufWriter::new(target_file);

        while let Some(item) = stream.try_next().await? {
            if let Some((_, ref mut digest, _)) = hasher {
                digest.update(&item);
            }

            tokio::io::copy(&mut item.as_ref(), &mut target_writer)
                .await?;
        }

        if let Some((_, hasher, content)) = hasher {
            let hash = hex::encode(&hasher.finalize());

            if hash != content {
                bail!("Mismatched hash!");
            }
        }

        if let Some(cache_destination) = cache_destination {
            tokio::fs::create_dir_all(
                output_destination.parent().ok_or(anyhow!("No parent"))?,
            )
            .await
            .with_context(|| format!("Create parent dirs: {}", output_destination.display()))?;
            tokio::fs::copy(&cache_destination, &output_destination).await?;
        }

        println!("Downloaded {}", metadata.filename);

        Ok(StepResult::Continue)
    }
}

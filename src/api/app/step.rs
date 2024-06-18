use std::path::Path;

use anyhow::{anyhow, bail, Result};
use tokio_stream::StreamExt;

use crate::api::{
    models::Addon,
    step::{CacheLocation, Step, StepResult},
    utils::hashing::get_best_hash,
};

use super::App;

impl App {
    pub async fn execute_steps(&self, dir: &Path, steps: &[Step]) -> Result<()> {
        for step in steps {
            let res = self.execute_step(dir, step).await?;

            if res == StepResult::Skip {
                break;
            }
        }

        Ok(())
    }

    pub async fn execute_step(&self, dir: &Path, step: &Step) -> Result<StepResult> {
        match step {
            Step::CacheCheck(metadata) => {
                if let Some(cache) = &metadata.cache {}

                Ok(StepResult::Continue)
            }

            Step::Download { url, metadata } => {
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

                let mut hasher = get_best_hash(&metadata.hashes)
                    .map(|(format, content)| (format, format.get_digest(), content));

                let target_destination = cache_destination.as_ref().unwrap_or(&output_destination);
                tokio::fs::create_dir_all(target_destination.parent().ok_or(anyhow!("No parent"))?)
                    .await?;

                let target_file = tokio::fs::File::create(&target_destination).await?;
                let mut target_writer = tokio::io::BufWriter::new(target_file);

                while let Some(item) = stream.try_next().await? {
                    if let Some((_, ref mut digest, _)) = hasher {
                        digest.update(&item);
                    }

                    tokio::io::copy(&mut item.as_ref(), &mut target_writer).await?;
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
                    .await?;
                    tokio::fs::copy(&cache_destination, &output_destination).await?;
                }

                println!("Downloaded {}", metadata.filename);

                Ok(StepResult::Continue)
            }

            Step::Execute => Ok(StepResult::Continue),
        }
    }
}

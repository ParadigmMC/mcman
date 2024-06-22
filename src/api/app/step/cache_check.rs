use std::path::Path;

use anyhow::Result;
use futures::{StreamExt, TryStreamExt};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::ReaderStream;

use crate::api::{app::App, step::{FileMeta, StepResult}};

impl App {
    // cache | output | to do
    //   x   |   x    | StepResult::Skip
    //   x   |        | copy from cache
    //       |   x    | StepResult::Continue
    //       |        | StepResult::Continue
    pub(super) async fn execute_step_cache_check(&self, dir: &Path, metadata: &FileMeta) -> Result<StepResult> {
        let output_path = dir.join(&metadata.filename);

        let Some(cached_path) = self.cache.loc(metadata.cache.as_ref()) else {
            return Ok(StepResult::Continue);
        };

        if !cached_path.try_exists()? {
            return Ok(StepResult::Continue);
        }

        let cached_meta = cached_path.metadata()?;
        let cached_size = cached_meta.len();

        let output_size = if output_path.try_exists()? {
            let meta = output_path.metadata()?;
            Some(meta.len())
        } else {
            None
        };

        if let Some(output_size) = output_size {
            // if output file exists...

            if output_size != cached_size || metadata.size.is_some_and(|x| x != output_size) {
                // size mismatch
                // TODO: print warning
                println!("WARNING size mismatch: {}", metadata.filename);
                tokio::fs::remove_file(&output_path).await?;
                //return Ok(StepResult::Continue);
            } else {
                let hasher = metadata.get_hasher();
                if let Some((format, mut hasher, content)) = hasher {
                    let output_file = File::open(&output_path).await?;
                    let mut stream = ReaderStream::new(output_file);

                    while let Some(item) = stream.try_next().await? {
                        hasher.update(&item);
                    }

                    let hash = hex::encode(&hasher.finalize());
                    
                    if content == hash {
                        // size and hash match, skip rest of the steps
                        // TODO: print info
                        println!("Skipping (output hash matches) {}", metadata.filename);
                        return Ok(StepResult::Skip);
                    } else {
                        // hash mismatch
                        // TODO: print warning
                        println!("WARNING Hash mismatch: {}", metadata.filename);
                        tokio::fs::remove_file(&output_path).await?;
                    }
                } else {
                    // FileInfo doesn't have any hashes
                    // so we must check from cache
                    // return skip if equal, do nothing otherwise to fallback copyfromcache
                    let target_file = File::open(&output_path).await?;
                    let cached_file = File::open(&cached_path).await?;

                    let mut target_stream = ReaderStream::new(target_file);
                    let mut cached_stream = ReaderStream::new(cached_file);

                    let is_equal = loop {
                        match (target_stream.next().await, cached_stream.next().await) {
                            (Some(Ok(a)), Some(Ok(b))) => {
                                if a != b {
                                    break false;
                                }
                            },
                            (None, None) => break true,
                            _ => break false,
                        }
                    };

                    if is_equal {
                        // TODO: print info
                        println!("Skipping (eq cached) {}", metadata.filename);
                        return Ok(StepResult::Skip);
                    }
                }
            }
        }

        // == Copying from cache ==

        let mut hasher = metadata.get_hasher();

        let target_file = File::create(&output_path).await?;
        let mut target_writer = BufWriter::new(target_file);

        let cached_file = File::open(&cached_path).await?;
        let mut stream = ReaderStream::new(cached_file);

        while let Some(item) = stream.try_next().await? {
            if let Some((_, ref mut digest, _)) = hasher {
                digest.update(&item);
            }

            tokio::io::copy(&mut item.as_ref(), &mut target_writer).await?;
        }

        if let Some((_, hasher, content)) = hasher {
            let hash = hex::encode(&hasher.finalize());

            if hash != content {
                // TODO: print warning
                println!("WARNING Hash Mismatch on CacheCopy: {}", metadata.filename);
                tokio::fs::remove_file(&output_path).await?;
                tokio::fs::remove_file(&cached_path).await?;
                return Ok(StepResult::Continue);
            }
        }

        println!("Copied: {}", metadata.filename);
        Ok(StepResult::Skip)
    }
}

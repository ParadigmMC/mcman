use std::{path::PathBuf, time::Duration, fmt::Debug};

use anyhow::{Result, bail, Context};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use tokio::{fs::File, io::BufWriter};
use digest::{Digest, DynDigest};

use crate::{App, Resolvable, CacheStrategy, ResolvedFile};

impl App {
    pub async fn download(
        &self,
        resolvable: &(impl Resolvable + ToString + Debug),
        destination: PathBuf,
        progress_bar: ProgressBar,
    ) -> Result<ResolvedFile> {
        progress_bar.set_style(ProgressStyle::with_template("{spinner:.blue} {prefix} {msg}...")?);
        progress_bar.set_prefix("Resolving");
        progress_bar.set_message(resolvable.to_string());
        progress_bar.enable_steady_tick(Duration::from_millis(250));

        let resolved = resolvable.resolve_source(&self).await
            .context(format!("Resolving {resolvable:#?}"))?;

        let cached_file = match &resolved.cache {
            CacheStrategy::File { namespace, path } => {
                if let Some(cache) = self.get_cache(namespace) {
                    if cache.exists(path) {
                        Some(cache.path(path))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            CacheStrategy::Indexed { .. } => todo!(),
            CacheStrategy::None => None,
        };

        let hasher_name = resolved.hashes
            .get_key_value("sha512")
            .or(resolved.hashes.get_key_value("sha256"))
            .or(resolved.hashes.get_key_value("md5"))
            .or(resolved.hashes.get_key_value("sha1"))
            .map(|h| h.0);

        let mut hasher = if let Some(name) = hasher_name {
            let digester: Box<dyn DynDigest> = match name.as_str() {
                "sha256" => Box::new(Sha256::new()),
                "sha512" => Box::new(Sha512::new()),
                "sha1" => Box::new(Sha1::new()),
                "md5" => Box::new(Md5::new()),
                _ => unreachable!(),
            };

            let hash = resolved.hashes[name].clone();

            Some((digester, hash))
        } else {
            None
        };

        let file_path = destination.join(&resolved.filename);

        tokio::fs::create_dir_all(file_path.parent().unwrap()).await
            .context(format!("Creating parent directories of '{}'", file_path.to_string_lossy()))?;
        let target_file = File::create(&file_path).await
            .context(format!("Creating destination file at '{}'", file_path.to_string_lossy()))?;

        if file_path.exists() {
            let meta = target_file.metadata().await
                .context(format!("Getting metadata of file '{}'", file_path.to_string_lossy()))?;
            if meta.is_dir() {
                bail!("'{}' is a directory and not a file", file_path.to_string_lossy());
            }

            let size_matches = if let Some(size) = resolved.size {
                meta.len() == size
            } else {
                true
            };

            // TODO: optionally check hashes for existing file

            if size_matches {
                // file already there and is ok
                return Ok(resolved);
            }
        }

        if let Some(cached) = cached_file {
            let cached_size = cached.metadata()
                .context(format!("Getting metadata of cached file at '{}'", cached.to_string_lossy()))?.len();

            if let Some(size) = resolved.size {
                if cached_size != size {
                    bail!("Cached file size is wrong! expected: {size}, actual: {cached_size}, path: {}", cached.to_string_lossy());
                }
            }

            //progress_bar.disable_steady_tick();
            // TODO: progressbar (i couldnt do it :c)
            //progress_bar.set_style(ProgressStyle::with_template("{prefix:.green.bold} {msg} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?);
            progress_bar.set_prefix("Copying");

            let mut cache_file = File::open(&cached).await
                .context(format!("Opening file '{}' from cache dir", cached.to_string_lossy()))?;
            let mut file_writer = BufWriter::new(target_file);
            
            tokio::io::copy(&mut cache_file, &mut file_writer).await
                .context(format!("Copying cached file
                -> From: {}
                -> To: {}", cached.to_string_lossy(), file_path.to_string_lossy()))?;

            // TODO: hash checks etc
        
            progress_bar.set_style(ProgressStyle::with_template("{prefix:.green.bold} {msg}")?);
            progress_bar.set_prefix("Copied");
            progress_bar.finish();
        } else {
            progress_bar.set_prefix("Fetching");
            progress_bar.set_message(resolved.filename.clone());

            let response = self.http_client.get(&resolved.url)
                .send()
                .await?
                .error_for_status()?;

            let content_length = response.content_length();

            match (resolved.size, content_length) {
                (Some(size), Some(len)) => {
                    if size != len {
                        // TODO: pretty msg
                        progress_bar.println(format!("WARNING: content length is wrong! expected: {size}, actual: {len}"));
                    }

                    progress_bar.set_length(len);
                }
                (Some(size), None) | (None, Some(size)) => progress_bar.set_length(size),
                _ => {},
            }

            progress_bar.disable_steady_tick();
            progress_bar.set_style(ProgressStyle::with_template("{prefix:.green.bold} {msg} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?);
            progress_bar.set_prefix("Downloading");

            let mut file_writer = BufWriter::new(target_file);
        
            let mut stream = response.bytes_stream();
            while let Some(item) = stream.next().await {
                let item = item?;

                if let Some((ref mut digest, _)) = hasher {
                    digest.update(&item);
                }

                tokio::io::copy(&mut item.as_ref(), &mut file_writer).await
                    .context("Writing downloaded chunk")?;
        
                progress_bar.inc(item.len() as u64);
            }

            if let Some((digest, resolved_hash)) = hasher {
                let stream_hash = base16ct::lower::encode_string(&digest.finalize());

                if resolved_hash != stream_hash {
                    // TODO: skipping checks etc
                    // also pretty msg
                    bail!("Mismatched hash!
                    Type: {}
                    Expected hash: {resolved_hash}
                    Real hash: {stream_hash}", hasher_name.unwrap());
                }
            }
        
            progress_bar.set_style(ProgressStyle::with_template("{prefix:.green.bold} {msg}")?);
            progress_bar.set_prefix("Downloaded");
            progress_bar.finish();
        }

        Ok(resolved)
    }
}

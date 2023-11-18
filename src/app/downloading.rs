use std::{path::PathBuf, time::Duration, fmt::Debug};

use anyhow::{Result, bail, Context};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use tokio::{fs::File, io::BufWriter};
use digest::{Digest, DynDigest};
use tokio_util::io::ReaderStream;

use crate::util::SelectItem;

use super::{App, Resolvable, CacheStrategy, ResolvedFile, Prefix, ProgressPrefix};

struct Bomb<T: FnMut()>(pub bool, pub T);

impl<T: FnMut()> Bomb<T> {
    pub fn defuse(&mut self) {
        self.0 = false;
    }
}

impl<T: FnMut()> Drop for Bomb<T> {
    fn drop(&mut self) {
        if self.0 {
            self.1()
        }
    }
}

impl App {
    pub async fn download(
        &self,
        resolvable: &(impl Resolvable + ToString + Debug),
        destination: PathBuf,
        progress_bar: ProgressBar,
    ) -> Result<ResolvedFile> {
        progress_bar.set_style(ProgressStyle::with_template("{spinner:.blue} {prefix} {msg}...")?);
        progress_bar.set_prefix(ProgressPrefix::Resolving);
        progress_bar.set_message(resolvable.to_string());
        progress_bar.enable_steady_tick(Duration::from_millis(250));

        let resolved = resolvable.resolve_source(&self).await
            .context(format!("Resolving {resolvable:#?}"))?;

        self.download_resolved(resolved, destination, progress_bar).await
    }

    pub fn resolve_cached_file(&self, cache: &CacheStrategy) -> Option<(PathBuf, bool)> {
        match cache {
            CacheStrategy::File { namespace, path } => {
                if let Some(cache) = self.get_cache(namespace) {
                    Some((cache.path(path), cache.exists(path)))
                } else {
                    None
                }
            }
            CacheStrategy::Indexed { .. } => todo!(),
            CacheStrategy::None => None,
        }
    }

    pub fn create_hasher(name: &str) -> Box<dyn DynDigest> {
        let digester: Box<dyn DynDigest> = match name {
            "sha256" => Box::new(Sha256::new()),
            "sha512" => Box::new(Sha512::new()),
            "sha1" => Box::new(Sha1::new()),
            "md5" => Box::new(Md5::new()),
            _ => unreachable!(),
        };

        digester
    }

    pub async fn download_resolved(
        &self,
        resolved: ResolvedFile,
        destination: PathBuf,
        progress_bar: ProgressBar,
    ) -> Result<ResolvedFile> {
        progress_bar.set_style(ProgressStyle::with_template("{spinner:.blue} {prefix} {msg}...")?);
        progress_bar.set_prefix(ProgressPrefix::Checking);
        progress_bar.enable_steady_tick(Duration::from_millis(250));

        // Some(Path) if file exists in cache
        let cached_file_path = self.resolve_cached_file(&resolved.cache);

        let hasher = Self::get_best_hash(&resolved.hashes);

        // if resolved has hashes, Some((hash name, dyndigest, hash value))
        let mut hasher = if let Some((name, hash)) = hasher {
            let digester: Box<dyn DynDigest> = App::create_hasher(&name);

            Some((name, digester, hash))
        } else {
            None
        };

        let validate_hash = |hasher: Option<(String, Box<dyn DynDigest>, String)>| {
            if let Some((hash_name, digest, resolved_hash)) = hasher {
                let stream_hash = base16ct::lower::encode_string(&digest.finalize());

                if resolved_hash == stream_hash {
                    self.dbg("hash check success");
                } else {
                    // TODO: skipping checks etc
                    // also pretty msg
                    bail!("Mismatched hash!
                    Type: {hash_name}
                    Expected hash: {resolved_hash}
                    Real hash: {stream_hash}");
                }
            }

            Ok(())
        };

        // dest. file path
        let file_path = destination.join(&resolved.filename);

        tokio::fs::create_dir_all(file_path.parent().unwrap()).await
            .context(format!("Creating parent directories of '{}'", file_path.to_string_lossy()))?;

        if file_path.exists() {
            let meta = file_path.metadata()
                .context(format!("Getting metadata of file '{}'", file_path.to_string_lossy()))?;
            if meta.is_dir() {
                let message = format!("'{}' is a directory and not a file", file_path.to_string_lossy());

                match self.select(&message, &[
                    SelectItem(0, "Delete folder and download".to_owned()),
                    SelectItem(1, "Skip file".to_owned()),
                    SelectItem(2, "Bail".to_owned()),
                ])? {
                    0 => {
                        tokio::fs::remove_dir_all(&file_path).await?;
                    },
                    1 => {
                        self.notify(Prefix::SkippedWarning, progress_bar.message());
                        return Ok(resolved)
                    },
                    2 => bail!(message),
                    _ => unreachable!(),
                }
            } else {
                let size_matches = if let Some(size) = resolved.size {
                    meta.len() == size
                } else {
                    true
                };

                // TODO: optionally check hashes for existing file

                if size_matches {
                    // file already there and is ok
                    self.notify(Prefix::Skipped, progress_bar.message());

                    return Ok(resolved);
                }
            }
        }

        let target_file = File::create(&file_path).await
            .context(format!("Creating destination file at '{}'", file_path.to_string_lossy()))?;

        // this bomb will explode (delete target_file) if its not defused (fn exits with Err)
        let mut bomb = Bomb(true, || {
            // i mean, atleast try right
            let _ = std::fs::remove_file(&file_path);
        });

        if let Some((cached, cached_size)) = match &cached_file_path {
            Some((cached, true)) => {
                let cached_size = cached.metadata()
                    .context(format!("Getting metadata of cached file at '{}'", cached.to_string_lossy()))?.len();

                match resolved.size {
                    Some(size) if size != cached_size => {
                        self.warn(format!(
                            "Cached file size is wrong!
- expected: {size}
- actual: {cached_size}
- path: {}",
                            cached.to_string_lossy()
                        ));
                        None
                    }
                    _ => Some((cached, cached_size)),
                }
            },
            _ => None,
        } {
            progress_bar.disable_steady_tick();
            progress_bar.set_style(ProgressStyle::with_template("{prefix:.blue.bold} {msg} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?);
            progress_bar.set_length(cached_size);
            progress_bar.set_prefix(ProgressPrefix::Copying);

            let mut cache_file = File::open(&cached).await
                .context(format!("Opening file '{}' from cache dir", cached.to_string_lossy()))?;
            let mut file_writer = BufWriter::new(target_file);

            let mut stream = ReaderStream::new(cache_file);
            while let Some(item) = stream.next().await {
                let item = item?;

                if let Some((_, ref mut digest, _)) = hasher {
                    digest.update(&item);
                }

                tokio::io::copy(&mut item.as_ref(), &mut file_writer).await
                    .context(format!("Copying cached file
                    -> From: {}
                    -> To: {}", cached.to_string_lossy(), file_path.to_string_lossy()))?;
        
                progress_bar.inc(item.len() as u64);
            }

            // TODO: retry downloading if fails
            validate_hash(hasher)?;
        
            self.notify(Prefix::Copied, resolved.filename.clone());
            progress_bar.finish_and_clear();
        } else {
            progress_bar.set_prefix(ProgressPrefix::Fetching);
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
                        self.warn(format!("content length is wrong! expected: {size}, actual: {len}"));
                    }

                    progress_bar.set_length(len);
                }
                (Some(size), None) | (None, Some(size)) => progress_bar.set_length(size),
                _ => {},
            }

            progress_bar.disable_steady_tick();
            progress_bar.set_style(ProgressStyle::with_template("{prefix:.blue.bold} {msg} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?);
            progress_bar.set_prefix(ProgressPrefix::Downloading);

            // if file can be cached, BufWriter to the file in cache dir
            // otherwise BufWriter to output file
            let mut file_writer = BufWriter::new(if let Some((path, _exists)) = cached_file_path {
                tokio::fs::create_dir_all(path.parent().unwrap()).await?;
                File::create(path).await?
            } else {
                target_file
            });
        
            let mut stream = response.bytes_stream();
            while let Some(item) = stream.next().await {
                let item = item?;

                if let Some((_, ref mut digest, _)) = hasher {
                    digest.update(&item);
                }

                tokio::io::copy(&mut item.as_ref(), &mut file_writer).await
                    .context("Writing downloaded chunk")?;
        
                progress_bar.inc(item.len() as u64);
            }

            validate_hash(hasher)?;

            // if we downloaded to cache instead of output above, copy the file to output
            // small todo: maybe write to both while downloading?
            if let Some(cached_file_path) = match &resolved.cache {
                CacheStrategy::File { namespace, path } => self.get_cache(namespace).map(|c| c.path(path)),
                CacheStrategy::Indexed { .. } => todo!(),
                CacheStrategy::None => None,
            } {
                progress_bar.set_style(ProgressStyle::with_template("{spinner:.blue} {prefix} {msg}...")?);
                progress_bar.set_prefix(ProgressPrefix::Copying);

                tokio::fs::copy(cached_file_path, &file_path).await?;
            }
        
            self.notify(Prefix::Downloaded, resolved.filename.clone());
            progress_bar.finish_and_clear();
        }

        // succeeded, so defuse
        bomb.defuse();

        Ok(resolved)
    }
}

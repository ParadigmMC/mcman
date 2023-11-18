use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use futures::StreamExt;
use indicatif::ProgressBar;
use sha2::Sha256;
use digest::{Digest, DynDigest};
use tokio::{fs::File, io::{AsyncWrite, AsyncRead}};
use tokio_util::io::ReaderStream;

use super::{App, ResolvedFile};

impl App {
    pub fn get_best_hash(hashes: &HashMap<String, String>) -> Option<(String, String)> {
        hashes
            .get_key_value("sha512")
            .or(hashes.get_key_value("sha256"))
            .or(hashes.get_key_value("md5"))
            .or(hashes.get_key_value("sha1"))
            .map(|(k, v)| (k.clone(), v.clone()))
    }

    pub async fn hash_resolved_file(
        &self,
        resolved: &ResolvedFile,
    ) -> Result<(String, String)> {
        if let Some(pair) = Self::get_best_hash(&resolved.hashes) {
            Ok(pair)
        } else {
            // calculate hash manually

            let (file_path, is_temp) = match self.resolve_cached_file(&resolved.cache) {
                Some((path, true)) => {
                    // file exists in cache dir
                    (path, false)
                }
                
                _ => {
                    // either can't cache or isnt in cache dir
                    // download it
                    self.download_resolved(resolved.clone(), PathBuf::from("."), ProgressBar::new_spinner()).await?;
                    (PathBuf::from(".").join(&resolved.filename), true)
                }
            };

            let preferred_hash = "sha256";
            let mut digester = Self::create_hasher(preferred_hash);

            let pb = self.multi_progress.add(ProgressBar::new_spinner()
                .with_message(format!("Calculating hash for {}", resolved.filename)));

            let mut file = File::open(&file_path).await
                .context(format!("Opening file '{}'", file_path.display()))?;

            let mut stream = ReaderStream::new(file);
            while let Some(item) = stream.next().await {
                let item = item?;
                digester.update(&item);
            }

            if is_temp {
                pb.set_message("Cleaning up...");

                tokio::fs::remove_file(&file_path)
                    .await.context(format!("Deleting {}", file_path.display()))?;
            }

            pb.finish_and_clear();

            let stream_hash = base16ct::lower::encode_string(&digester.finalize());

            Ok((preferred_hash.to_owned(), stream_hash))
        }
    }

    pub async fn copy_with_hashing<R: AsyncRead + std::marker::Unpin, W: AsyncWrite + std::marker::Unpin>(
        source: &mut R,
        dest: &mut W,
        mut digester: Box<dyn DynDigest>,
    ) -> Result<String> {
        let mut stream = ReaderStream::new(source);
        while let Some(item) = stream.next().await {
            let item = item?;

            digester.update(&item);

            tokio::io::copy(&mut item.as_ref(), dest).await?;
        }

        Ok(base16ct::lower::encode_string(&digester.finalize()))
    }

    pub fn hash_sha256(contents: &str) -> String {
        let mut hasher = Sha256::new();
    
        Digest::update(&mut hasher, contents);
    
        base16ct::lower::encode_string(&hasher.finalize())
    }
}

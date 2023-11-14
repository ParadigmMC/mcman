use std::{path::PathBuf, time::Duration, collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result, Context};
use console::style;
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle, ProgressIterator};
use rpackwiz::model::{Mod, ModUpdate, ModDownload, HashFormat, DownloadMode, Pack, PackIndex};
use serde::de::DeserializeOwned;

use crate::{app::{App, Resolvable, ResolvedFile, CacheStrategy}, model::Downloadable};

#[derive(Debug, Clone)]
pub enum FileProvider {
    LocalFolder(PathBuf),
    RemoteURL(reqwest::Client, reqwest::Url),
}

impl FileProvider {
    pub async fn parse_toml<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        match self {
            Self::LocalFolder(folder) => {
                let str = tokio::fs::read_to_string(folder.join(path)).await?;
                Ok(toml::from_str(&str)?)
            }
            Self::RemoteURL(http_client, url) => {
                let contents = http_client
                    .get(url.join(path)?)
                    .send()
                    .await?
                    .error_for_status()?
                    .text()
                    .await?;
            
                Ok(toml::from_str(&contents)?)
            }
        }
    }
}

pub struct PackwizInterop<'a>(pub &'a mut App);

impl<'a> PackwizInterop<'a> {
    pub fn get_file_provider(&self, s: &str) -> Result<FileProvider> {
        Ok(if s.starts_with("http") {
            FileProvider::RemoteURL(self.0.http_client.clone(), s.try_into()?)
        } else {
            FileProvider::LocalFolder(s.into())
        })
    }

    pub async fn import_all(
        &mut self,
        from: &str,
    ) -> Result<()> {
        self.import_from_source(self.get_file_provider(from)?).await
    }

    pub async fn import_from_source(
        &mut self,
        source: FileProvider,
    ) -> Result<()> {
        let progress_bar = self.0.multi_progress.add(ProgressBar::new_spinner()
            .with_finish(ProgressFinish::WithMessage("Imported".into())));
        progress_bar.set_style(ProgressStyle::with_template("{spinner:.blue} {msg}")?);
        progress_bar.set_message("Reading pack.toml...");
        progress_bar.enable_steady_tick(Duration::from_millis(250));
        
        let pack: Pack = source.parse_toml("pack.toml").await?;

        self.0.server.fill_from_map(&pack.versions);
        
        progress_bar.set_message("Reading pack index...");
        
        let index: PackIndex = source.parse_toml(&pack.index.file).await?;

        progress_bar.set_message(format!("Importing {} files...", index.files.len()));

        let pb = self.0.multi_progress.insert_after(&progress_bar, ProgressBar::new(index.files.len() as u64))
            .with_style(ProgressStyle::with_template("  {prefix:.blue.bold} {msg} [{wide_bar:.cyan/blue}] {pos}/{len}")?)
            .with_prefix("Importing");

        for file in index.files.iter().progress_with(pb.clone()) {
            pb.set_message(file.file.clone());
            if file.metafile {
                if file.file.starts_with("mods") {
                    let modpw: Mod = source.parse_toml(&file.file).await?;

                    let dl = self.from_mod(&modpw).await?;
                    self.0.println(format!(
                        "{} {}",
                        style("      Imported").green().bold(),
                        dl.to_short_string()
                    ))?;

                    self.0.server.mods.push(dl);
                } else {
                    // TODO: ???
                    self.0.warn(format!("unsupported metafile: {} - please open an issue at github", file.file))?;
                }
            } else {
                let output_path = self.0.server.path.join("config").join(&file.file);
                
                match &source {
                    FileProvider::LocalFolder(folder) => {
                        tokio::fs::copy(folder.join(&file.file), output_path).await?;
                    }
                    FileProvider::RemoteURL(_, url) => {
                        self.0.download_resolved(
                            ResolvedFile {
                                url: url.join(&file.file)?.as_str().to_owned(),
                                filename: file.file.split('/').last().unwrap().to_owned(),
                                cache: CacheStrategy::None,
                                size: None,
                                hashes: HashMap::from([
                                    (match index.hash_format {
                                        HashFormat::Md5 => "md5",
                                        HashFormat::Sha1 => "sha1",
                                        HashFormat::Sha256 => "sha256",
                                        HashFormat::Sha512 => "sha512",
                                        HashFormat::Curseforge => "murmur2",
                                        _ => unreachable!(),
                                    }.to_owned(), file.hash.clone())
                                ]),
                            },
                            output_path.parent().unwrap().to_path_buf(),
                            self.0.multi_progress.insert_after(&pb, ProgressBar::new_spinner())
                        ).await?;
                    }
                }
            }
        }

        progress_bar.finish_and_clear();
        self.0.success("Packwiz pack imported!")?;

        Ok(())
    }

    pub async fn from_mod(&self, m: &Mod) -> Result<Downloadable> {
        if let Some(dl) = self.from_hash(&m.download).await? {
            Ok(dl)
        } else if let Some(dl) = self.from_mod_update(&m.update)? {
            Ok(dl)
        } else {
            self.0.dl_from_string(&m.download
                .url
                .clone()
                .ok_or(anyhow!("Download URL not present for mod: {m:#?}"))?)
                .await
                .context(format!("Importing mod: {m:#?}"))
        }
    }

    pub async fn from_hash(&self, down: &ModDownload) -> Result<Option<Downloadable>> {
        if !down.hash.is_empty() {
            let fmt = match down.hash_format {
                HashFormat::Sha512 => "sha512",
                HashFormat::Sha1 => "sha1",
                _ => return Ok(None),
            };

            Ok(match self.0.modrinth().version_from_hash(&down.hash, fmt).await {
                Ok(ver) => Some(Downloadable::Modrinth { id: ver.project_id.clone(), version: ver.id.clone() }),
                _ => None,
            })
        } else {
            Ok(None)
        }
    }

    pub fn from_mod_update(&self, mod_update: &Option<ModUpdate>) -> Result<Option<Downloadable>> {
        if let Some(upd) = mod_update {
            if let Some(mr) = &upd.modrinth {
                Ok(Some(Downloadable::Modrinth {
                    id: mr.mod_id.clone(),
                    version: mr.version.clone(),
                }))
            } else if let Some(cf) = &upd.curseforge {
                Ok(Some(Downloadable::CurseRinth {
                    id: cf.project_id.to_string(),
                    version: cf.file_id.to_string(),
                }))
            } else {
                // TODO clarify
                self.0.warn(format!("unknown mod update"))?;
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn to_mod(&self, dl: &Downloadable) -> Result<Mod> {
        let resolved = dl.resolve_source(&self.0).await?;

        // TODO: mod update metadata

        self.resolved_to_mod(&resolved).await
    }

    pub async fn resolved_to_mod(&self, resolved_file: &ResolvedFile) -> Result<Mod> {
        let hash = App::get_best_hash(&resolved_file.hashes);

        let (hash_format, hash) = match hash {
            Some(t) => t,
            None => {
                // TODO calculate hash manually (by cached file or download it and compute)
                todo!()
            }
        };

        Ok(Mod {
            filename: resolved_file.filename.clone(),
            name: resolved_file.filename.clone(),
            download: ModDownload {
                url: Some(resolved_file.url.clone()),
                hash,
                hash_format: match hash_format.as_str() {
                    "sha1" => HashFormat::Sha1,
                    "sha256" => HashFormat::Sha256,
                    "sha512" => HashFormat::Sha512,
                    "md5" => HashFormat::Md5,
                    "murmur2" => HashFormat::Curseforge,
                    _ => HashFormat::Md5,
                },
                mode: DownloadMode::Url,
            },
            ..Default::default()
        })
    }
}

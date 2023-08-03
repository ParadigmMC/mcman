use anyhow::{anyhow, bail, Context, Result};
use rpackwiz::model::{
    CurseforgeModUpdate, DownloadMode, HashFormat, Mod, ModDownload, ModOption, ModUpdate,
    ModrinthModUpdate, Side,
};

use crate::{
    model::Server,
    sources::{
        curserinth::{fetch_curserinth_project, fetch_curserinth_versions},
        modrinth::{fetch_modrinth_project, fetch_modrinth_versions, DependencyType},
    },
    util::{hash::get_hash_url, packwiz::PackwizExportOptions},
    Source,
};

use super::Downloadable;

impl Downloadable {
    pub async fn from_pw_mod(
        m: &Mod,
        http_client: &reqwest::Client,
        server: &Server,
    ) -> Result<Option<Self>> {
        Ok(Some(if let Some(upd) = &m.update {
            if let Some(mr) = &upd.modrinth {
                Self::Modrinth {
                    id: mr.mod_id.clone(),
                    version: mr.version.clone(),
                }
            } else if let Some(cf) = &upd.curseforge {
                Self::CurseRinth {
                    id: cf.project_id.to_string(),
                    version: cf.file_id.to_string(),
                }
            } else {
                println!("ERROR: UNKNOWN MOD UPDATE");
                return Ok(None); // Hell
            }
        } else {
            Self::from_url_interactive(
                http_client,
                server,
                &m.download
                    .url
                    .clone()
                    .ok_or(anyhow!("download url not present"))?,
                false,
            )
            .await
            .context("Resolving Downloadable from URL")?
        }))
    }

    #[allow(clippy::too_many_lines)] // xd
    pub async fn to_pw_mod(
        &self,
        http_client: &reqwest::Client,
        server: &Server,
        opts: &PackwizExportOptions,
        is_opt: Option<bool>,
        desc_override: &str,
    ) -> Result<Option<(String, Mod)>> {
        Ok(match &self {
            Self::Modrinth { id, version } => {
                let proj = fetch_modrinth_project(http_client, id).await?;

                let side = match (proj.server_side.clone(), proj.client_side.clone()) {
                    (DependencyType::Incompatible | DependencyType::Unsupported, _) => Side::Client,
                    (_, DependencyType::Incompatible | DependencyType::Unsupported) => Side::Server,
                    _ => Side::Both,
                };

                let versions = fetch_modrinth_versions(http_client, id, None).await?;

                let verdata = if version == "latest" {
                    versions.first()
                } else {
                    versions.iter().find(|&v| v.id == version.clone())
                };

                let Some(verdata) = verdata else {
                    bail!("Release '{version}' for project '{id}' not found");
                };

                let Some(file) = verdata.files.first() else {
                    bail!("No files for project '{id}' version '{version}'");
                };

                let hash = file
                    .hashes
                    .get("sha512")
                    .expect("modrinth to provide sha512 hashes")
                    .clone();

                let m = Mod {
                    name: proj.title,
                    side,
                    filename: file.filename.clone(),
                    download: ModDownload {
                        mode: DownloadMode::Url,
                        url: Some(file.url.clone()),
                        hash,
                        hash_format: HashFormat::Sha512,
                    },
                    option: ModOption {
                        optional: is_opt.unwrap_or(proj.client_side == DependencyType::Optional),
                        default: false,
                        description: Some(if desc_override.is_empty() {
                            proj.description
                        } else {
                            desc_override.to_owned()
                        }),
                    },
                    update: Some(ModUpdate {
                        modrinth: Some(ModrinthModUpdate {
                            mod_id: proj.id,
                            version: version.clone(),
                        }),
                        curseforge: None,
                    }),
                };

                Some((proj.slug + ".pw.toml", m))
            }

            Self::CurseRinth { id, version } => {
                let proj = fetch_curserinth_project(http_client, id).await?;

                let side = match (proj.server_side.clone(), proj.client_side.clone()) {
                    (DependencyType::Incompatible | DependencyType::Unsupported, _) => Side::Client,
                    (_, DependencyType::Incompatible | DependencyType::Unsupported) => Side::Server,
                    _ => Side::Both,
                };

                let versions = fetch_curserinth_versions(http_client, id, None).await?;

                let verdata = if version == "latest" {
                    versions.first()
                } else {
                    versions.iter().find(|&v| v.id == version.clone())
                };

                let Some(verdata) = verdata else {
                    bail!("Release '{version}' for project '{id}' not found");
                };

                let Some(file) = verdata.files.first() else {
                    bail!("No files for project '{id}' version '{version}'");
                };

                let hash = file
                    .hashes
                    .get("sha1")
                    .expect("curserinth to provide sha1 hashes from cf")
                    .clone();

                let m = Mod {
                    name: proj.title,
                    side,
                    filename: file.filename.clone(),
                    download: if opts.cf_usecdn {
                        ModDownload {
                            mode: DownloadMode::Url,
                            url: Some(file.url.clone()),
                            hash,
                            hash_format: HashFormat::Sha1,
                        }
                    } else {
                        ModDownload {
                            mode: DownloadMode::Curseforge,
                            url: None,
                            hash,
                            hash_format: HashFormat::Sha1,
                        }
                    },
                    option: ModOption {
                        optional: is_opt.unwrap_or(proj.client_side == DependencyType::Optional),
                        default: false,
                        description: Some(if desc_override.is_empty() {
                            proj.description
                        } else {
                            desc_override.to_owned()
                        }),
                    },
                    update: if opts.cf_usecdn {
                        None
                    } else {
                        Some(ModUpdate {
                            modrinth: None,
                            curseforge: Some(CurseforgeModUpdate {
                                file_id: verdata.id.parse()?,
                                project_id: verdata.project_id.parse()?,
                            }),
                        })
                    },
                };

                Some((proj.slug + ".pw.toml", m))
            }

            Self::Url { url, desc, .. } => {
                let filename = self.get_filename(server, http_client).await?;

                let hash = get_hash_url(http_client, url).await?;

                let m = Mod {
                    name: filename.clone(),
                    side: Side::Both,
                    filename: filename.clone(),
                    download: ModDownload {
                        mode: DownloadMode::Url,
                        url: Some(url.clone()),
                        hash,
                        hash_format: HashFormat::Sha256,
                    },
                    option: ModOption {
                        optional: is_opt.unwrap_or(false),
                        default: false,
                        description: if desc_override.is_empty() {
                            desc.clone()
                        } else {
                            Some(desc_override.to_owned())
                        },
                    },
                    update: None,
                };

                Some((filename + ".pw.toml", m))
            }

            dl => {
                println!("WARNING: Cant make this a mod: {dl:#?}");
                None
            }
        })
    }
}

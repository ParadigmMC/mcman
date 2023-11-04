use anyhow::{Result, bail};

use crate::{model::Downloadable, util::SelectItem};

use super::App;

impl App {
    pub async fn dl_from_string(
        &self,
        s: &str
    ) -> Result<Downloadable> {
        if s.starts_with("http") {
            Ok(self.dl_from_url(s).await?)
        } else if s.contains(':') {
            match s.split_once(':').unwrap() {
                ("mr" | "modrinth", id) => {
                    let (id, version) = id.split_once(',').unwrap_or((id, "latest"));
                    Ok(Downloadable::Modrinth {
                        id: id.to_owned(),
                        version: version.to_owned(),
                    })
                }
                ("cr" | "cf" | "curseforge" | "curserinth", id) => {
                    let (id, version) = id.split_once(',').unwrap_or((id, "latest"));
                    Ok(Downloadable::CurseRinth {
                        id: id.to_owned(),
                        version: version.to_owned(),
                    })
                }
                ("hangar", id) => {
                    let (id, version) = id.split_once(',').unwrap_or((id, "latest"));
                    Ok(Downloadable::Hangar {
                        id: id.to_owned(),
                        version: version.to_owned(),
                    })
                }
                ("spigot" | "spiget", id) => {
                    let (id, version) = id.split_once(',').unwrap_or((id, "latest"));
                    Ok(Downloadable::Spigot {
                        id: id.to_owned(),
                        version: version.to_owned(),
                    })
                }
                ("ghrel" | "gh" | "github", id) => {
                    let (repo, tag) = id.split_once(',').unwrap_or((id, "latest"));
                    
                    Ok(Downloadable::GithubRelease {
                        repo: repo.to_owned(),
                        tag: tag.to_owned(),
                        asset: "first".to_owned(),
                    })
                }
                (ty, _) => bail!("Unknown identifier '{ty}'"),
            }
        } else {
            bail!("I dont know what to do with '{s}'...");
        }
    }

    pub async fn dl_from_url(
        &self,
        urlstr: &str
    ) -> Result<Downloadable> {
        let url = reqwest::Url::parse(urlstr)?;

        match (url.domain(), url.path_segments().map(|x| x.collect::<Vec<_>>()).unwrap_or_default().as_slice()) {
            // https://cdn.modrinth.com/data/{ID}/versions/{VERSION}/{FILENAME}
            (Some("cdn.modrinth.com"), ["data", id, "versions", version, _filename]) => {
                Ok(Downloadable::Modrinth {
                    id: id.to_owned().to_owned(),
                    version: version.to_owned().to_owned(),
                })
            }

            (Some("modrinth.com"), ["mod" | "plugin" | "datapack", id, rest @ ..]) => {
                let version = match rest {
                    ["version", v] => v.to_string(),
                    _ => {
                        let versions = self.modrinth().fetch_versions(id).await?;

                        if versions.is_empty() {
                            bail!("No compatible versions found");
                        }

                        let version = self.select("Select a version", &versions.iter().map(|v| {
                            SelectItem(v.clone(), if v.version_number == v.name {
                                v.version_number.clone()
                            } else {
                                format!(
                                    "[{}] {}",
                                    v.version_number,
                                    v.name,
                                )
                            })
                        }).collect::<Vec<_>>())?;

                        version.id.clone()
                    }
                };

                Ok(Downloadable::Modrinth {
                    id: id.to_owned().to_owned(),
                    version: version.to_owned().to_owned(),
                })
            }

            (Some("curserinth.kuylar.dev"), ["mod", id, rest @ ..]) => {
                let version = match rest {
                    ["version", v] => v.to_string(),
                    _ => {
                        let (versions, _) = self.curserinth().fetch_versions(id).await?;

                        if versions.is_empty() {
                            bail!("No compatible versions found");
                        }

                        let version = self.select("Select a version", &versions.iter().map(|v| {
                            SelectItem(v.clone(), if v.version_number == v.name {
                                v.version_number.clone()
                            } else {
                                format!(
                                    "[{}] {}",
                                    v.version_number,
                                    v.name,
                                )
                            })
                        }).collect::<Vec<_>>())?;

                        version.id.clone()
                    }
                };

                Ok(Downloadable::CurseRinth {
                    id: id.to_owned().to_owned(),
                    version: version.to_owned().to_owned(),
                })
            }

            // https://www.curseforge.com/minecraft/mc-mods/betterwithpatches
            (Some("www.curseforge.com"), ["minecraft", "mc-mods", id, rest @ ..]) => {
                let id = format!("mod__{id}");

                let version = match rest {
                    [_, ver] => ver.to_string(),
                    _ => {
                        let (versions, _) = self.curserinth().fetch_versions(&id).await?;

                        if versions.is_empty() {
                            bail!("No compatible versions found");
                        }

                        let version = self.select("Select a version", &versions.iter().map(|v| {
                            SelectItem(v.clone(), if v.version_number == v.name {
                                v.version_number.clone()
                            } else {
                                format!(
                                    "[{}] {}",
                                    v.version_number,
                                    v.name,
                                )
                            })
                        }).collect::<Vec<_>>())?;

                        version.id.clone()
                    }
                };

                Ok(Downloadable::CurseRinth {
                    id: id.to_owned().to_owned(),
                    version: version.to_owned().to_owned(),
                })
            }

            // https://www.spigotmc.org/resources/http-requests.101253/
            (Some("www.spigotmc.org"), ["resources", id]) => {
                Ok(Downloadable::Spigot { id: id.to_string(), version: "latest".to_owned() })
            }

            // https://github.com/{owner}/{repo}/releases/{'tag'|'download'}/{tag}/{filename}
            (Some("github.com"), [owner, repo_name, rest @ ..]) => {
                let repo = format!("{owner}/{repo_name}");

                let (tag, asset) = match rest {
                    ["releases", "tag" | "download", tag, filename @ ..] => {

                        (tag.to_string(), match filename {
                            [f] => Some(f.replace(tag, "${tag}")),
                            _ => None,
                        })
                    }

                    _ => {
                        let releases = self.github().fetch_releases(&repo).await?;

                        let version = self.select("Select a release", &vec![
                            SelectItem("latest".to_owned(), "Always use latest release".to_owned())
                        ].into_iter().chain(releases.iter().map(|r| {
                            SelectItem(r.tag_name.clone(), if r.tag_name == r.name {
                                r.name.clone()
                            } else {
                                format!(
                                    "[{}] {}",
                                    r.tag_name,
                                    r.name
                                )
                            })
                        })).collect::<Vec<_>>())?;

                        (version, None)
                    }
                };

                let asset = match asset {
                    Some(a) => a,
                    None => {
                        let rel = self.github().fetch_release(&repo, &tag).await?;

                        if rel.assets.len() <= 1 {
                            "first".to_owned()
                        } else {
                            match self.select("Which asset to use?", &vec![
                                SelectItem(Some("first".to_owned()), format!(
                                    "Use the first asset ('{}' for '{}')",
                                    rel.assets[0].name, rel.tag_name
                                ))
                            ].into_iter().chain(rel.assets.iter().map(|a| {
                                SelectItem(Some(a.name.clone()), a.name.to_owned())
                            })).chain(vec![
                                SelectItem(None, "Set manually".to_string())
                            ]).collect::<Vec<_>>())? {
                                Some(a) => a,
                                None => self.prompt_string("Enter asset name")?,
                            }
                        }
                    }
                };

                Ok(Downloadable::GithubRelease { repo, tag: tag.to_string(), asset })
            }
            
            _ => {
                let selection = self.select(&urlstr, &vec![
                    SelectItem(0, "Add as Custom URL".to_owned()),
                    SelectItem(1, "Add as Jenkins".to_owned()),
                ])?;

                match selection {
                    0 => {
                        let inferred = urlstr
                            .split('?')
                            .next()
                            .unwrap_or(urlstr)
                            .split('/')
                            .last()
                            .unwrap();

                        let input = self.prompt_string_filled("Filename?", inferred)?;
                        let desc = self.prompt_string_default("Optional description/comment?", "")?;

                        Ok(Downloadable::Url {
                            url: urlstr.to_owned(),
                            filename: Some(input),
                            desc: if desc.is_empty() { None } else { Some(desc) },
                        })
                    }
                    1 => {
                        // TODO: ...
                        let j_url = if self.confirm(&format!("Is the Jenkins URL 'https://{}'?", url.domain().unwrap()))?
                        {
                            format!("https://{}", url.domain().unwrap())
                        } else {
                            self.prompt_string_filled("Enter Jenkins URL", &format!("https://{}", url.domain().unwrap()))?
                        };

                        let inferred_job = {
                            let mut job = String::new();

                            if let Some(mut segments) = url.path_segments() {
                                loop {
                                    if segments.next().unwrap_or_default() == "job" {
                                        if let Some(job_name) = segments.next() {
                                            if !job.is_empty() {
                                                job += "/";
                                            }

                                            job += job_name;
                                        } else {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }

                            job
                        };

                        let job = self.prompt_string_filled("Job", &inferred_job)?;
                        let build = self.prompt_string_filled("Build", "latest")?;
                        let artifact = self.prompt_string_filled("Artifact", "first")?;

                        Ok(Downloadable::Jenkins {
                            url: j_url,
                            job,
                            build,
                            artifact,
                        })
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

use anyhow::{Result, anyhow, bail};
use dialoguer::{Select, theme::ColorfulTheme, Input, Confirm};
use reqwest::Url;

use crate::model::Server;

use super::{Downloadable, sources::modrinth::{fetch_modrinth_versions, ModrinthVersion}};

impl Downloadable {
    pub async fn from_url_interactive(
        client: &reqwest::Client,
        server: &Server,
        urlstr: &str,
    ) -> Result<Self> {
        let url = Url::parse(urlstr)?;
        match url.domain() {
            Some("cdn.modrinth.com") => {
                // https://cdn.modrinth.com/data/{ID}/versions/{VERSION}/{FILENAME}
                let invalid_url = || anyhow!("Invalid Modrinth CDN URL");

                let segments: Vec<&str> = url
                    .path_segments()
                    .ok_or_else(invalid_url)?
                    .collect();
                let id = segments
                    .get(1)
                    .ok_or_else(invalid_url)?;
                let version = segments
                    .get(3)
                    .ok_or_else(invalid_url)?;
                //let filename = segments.get(4).ok_or_else(|| anyhow!("Invalid Modrinth CDN URL"))?;

                if segments.first() != Some(&"data") || segments.get(2) != Some(&"versions") {
                    Err(invalid_url())?;
                }

                Ok(Self::Modrinth {
                    id: id.to_owned().to_owned(),
                    version: version.to_owned().to_owned(),
                })
            }
            Some("modrinth.com") => {
                let invalid_url = || anyhow!("Invalid Modrinth CDN URL");

                let segments: Vec<&str> = url
                    .path_segments()
                    .ok_or_else(invalid_url)?
                    .collect();

                if segments.first() != Some(&"mod") || segments.first() != Some(&"plugin") {
                    Err(invalid_url())?;
                };

                let id = segments
                    .get(1)
                    .ok_or_else(invalid_url)?.to_owned().to_owned();

                let versions: Vec<ModrinthVersion> = fetch_modrinth_versions(client, &id, None).await?
                    .into_iter()
                    .filter(|v| v.game_versions.contains(&server.mc_version))
                    .collect();

                let version = if let Some(&"version") = segments.get(2) {
                    let ver_num = segments.get(3)
                        .ok_or_else(invalid_url)?.to_owned();

                    versions.into_iter().find(|v| v.version_number == ver_num)
                        .ok_or(anyhow!("Couldn't find the version '{ver_num}'"))?
                } else {
                    if versions.is_empty() {
                        bail!("No compatible versions in modrinth project");
                    }

                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Which version?")
                        .default(0)
                        .items(&versions.iter().map(|v| {
                            let num = &v.version_number;
                            let name = &v.name;
                            let compat = v.loaders.join(",");
                            format!("[{num}] {name} / {compat}")
                        }).collect::<Vec<String>>())
                        .interact()
                        .unwrap();

                    versions[selection].clone()
                };

                Ok(Self::Modrinth {
                    id,
                    version: version.id,
                })
            }
            Some("www.spigotmc.org") => {
                // https://www.spigotmc.org/resources/http-requests.101253/

                let segments: Vec<&str> = url
                    .path_segments()
                    .ok_or_else(|| anyhow!("Invalid url"))?
                    .collect();

                if segments.first() != Some(&"resources") {
                    Err(anyhow!("Invalid Spigot Resource URL"))?;
                }

                let id = segments
                    .get(1)
                    .ok_or_else(|| anyhow!("Invalid Spigot Resource URL"))?;

                Ok(Downloadable::Spigot {
                    id: id.to_owned().to_owned(),
                })
            }
            Some(_) | None => {
                let items = vec![
                    "Add as Custom URL",
                    "Add as Jenkins",
                    "Nevermind, cancel",
                ];
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .items(&items)
                    .default(0)
                    .interact_opt()?;

                match selection {
                    Some(0) => {
                        let inferred = urlstr
                            .split('?')
                            .next()
                            .unwrap_or(urlstr)
                            .split('/')
                            .last()
                            .unwrap();

                        let input: String = Input::new()
                            .with_prompt("Filename?")
                            .with_initial_text(inferred)
                            .default(inferred.into())
                            .interact_text()?;

                        Ok(Self::Url { url: urlstr.to_owned(), filename: Some(input.to_owned()) })
                    },
                    Some(1) => {
                        // TODO: make it better
                        println!(" >>> {}", url.domain().unwrap());
                        let j_url = if Confirm::new().with_prompt("Is this the correct jenkins server url?").interact()? {
                            url.domain().unwrap().to_owned()
                        } else {
                            Input::<String>::new()
                                .with_prompt("Jenkins URL:")
                                .with_initial_text(urlstr)
                                .default(urlstr.into())
                                .interact_text()?
                        };

                        let job: String = Input::new()
                            .with_prompt("Job:")
                            .interact_text()?;
                        
                        let build: String = Input::new()
                            .with_prompt("Build:")
                            .with_initial_text("latest")
                            .default("latest".into())
                            .interact_text()?;
                        
                        let artifact: String = Input::new()
                            .with_prompt("Artifact:")
                            .with_initial_text("first")
                            .default("first".into())
                            .interact_text()?;

                        Ok(Self::Jenkins {
                            url: j_url,
                            job,
                            build,
                            artifact,
                        })
                    },
                    None | Some(_) => bail!("Cancelled"),
                }
            }
        }
    }
}
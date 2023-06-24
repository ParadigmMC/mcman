use anyhow::{anyhow, bail, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use reqwest::Url;

use crate::model::Server;

use super::{
    sources::{
        github::fetch_github_releases,
        modrinth::{fetch_modrinth_versions, ModrinthVersion},
    },
    Downloadable,
};

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

                let segments: Vec<&str> = url.path_segments().ok_or_else(invalid_url)?.collect();
                let id = segments.get(1).ok_or_else(invalid_url)?;
                let version = segments.get(3).ok_or_else(invalid_url)?;
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
                let invalid_url = |r| anyhow!("Invalid Modrinth project URL: {r}");

                let segments: Vec<&str> = url
                    .path_segments()
                    .ok_or_else(|| invalid_url("couldn't split to segments"))?
                    .collect();

                if segments.first().is_none()
                    || !vec!["mod", "plugin"].contains(segments.first().unwrap())
                {
                    Err(invalid_url("must start with /mod or /plugin"))?;
                };

                let id = segments
                    .get(1)
                    .ok_or_else(|| invalid_url("no id"))?
                    .to_owned()
                    .to_owned();

                let versions: Vec<ModrinthVersion> = fetch_modrinth_versions(client, &id, None)
                    .await?
                    .into_iter()
                    .filter(|v| v.game_versions.contains(&server.mc_version))
                    .collect();

                let version = if let Some(&"version") = segments.get(2) {
                    let ver_num = segments
                        .get(3)
                        .ok_or_else(|| invalid_url("no version number in url"))?
                        .to_owned();

                    versions
                        .into_iter()
                        .find(|v| v.version_number == ver_num)
                        .ok_or(anyhow!("Couldn't find the version '{ver_num}'"))?
                } else {
                    if versions.is_empty() {
                        bail!("No compatible versions in modrinth project");
                    }

                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Which version?")
                        .default(0)
                        .items(
                            &versions
                                .iter()
                                .map(|v| {
                                    let num = &v.version_number;
                                    let name = &v.name;
                                    let compat = v.loaders.join(",");
                                    format!("[{num}] {name} / {compat}")
                                })
                                .collect::<Vec<String>>(),
                        )
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

                if segments.first().is_none() || *segments.first().unwrap() != "resources" {
                    Err(anyhow!("Invalid Spigot Resource URL"))?;
                }

                let id = segments
                    .get(1)
                    .ok_or_else(|| anyhow!("Invalid Spigot Resource URL"))?;

                Ok(Downloadable::Spigot {
                    id: id.to_owned().to_owned(),
                })
            }
            // the code under this domain is awful.. srry
            Some("github.com") => {
                // https://github.com/IPTFreedom/TotalFreedomMod/releases/tag/2022.06.08-IPT
                // https://github.com/IPTFreedom/TotalFreedomMod/releases/download/2022.06.08-IPT/TotalFreedomMod-2022.06.08-IPT.jar

                let mut segments = url.path_segments().ok_or_else(|| anyhow!("Invalid url"))?;

                let repo = [
                    segments
                        .next()
                        .ok_or(anyhow!("Couldn't find the repo from url"))?,
                    segments
                        .next()
                        .ok_or(anyhow!("Couldn't find the repo from url"))?,
                ]
                .join("/");

                let mut tag_opt = None;
                let mut file_opt = None;

                if segments.next() == Some("releases") {
                    match segments.next() {
                        Some("tag") => {
                            let invalid_url = || anyhow!("Invalid github tag url");

                            let tag = segments.next().ok_or_else(invalid_url)?;

                            tag_opt = Some(tag.to_owned());

                            println!("> Using release {tag}");
                        }
                        Some("download") => {
                            let invalid_url = || anyhow!("Invalid github release download url");

                            let tag = segments.next().ok_or_else(invalid_url)?;

                            tag_opt = Some(tag.to_owned());

                            println!("> Using release '{tag}'");

                            let file = segments.next().ok_or_else(invalid_url)?;

                            file_opt = Some(file);

                            println!("> Using asset '{tag}'");
                        }
                        Some(p) => bail!("No idea what to do with releases/{p}"),
                        None => {}
                    }
                };

                let fetched_tags = fetch_github_releases(&repo, client).await?;

                let tag = if let Some(t) = tag_opt {
                    t
                } else {
                    let mut items = vec!["Always use latest release".to_owned()];

                    for r in &fetched_tags {
                        items.push(format!("Release {}", r.name));
                    }

                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Which release to use?")
                        .items(&items)
                        .default(0)
                        .interact_opt()?
                        .ok_or(anyhow!("Cancelled"))?;

                    if selection == 0 {
                        "latest".to_owned()
                    } else {
                        fetched_tags[selection - 1].tag_name.clone()
                    }
                };

                let mut idx = 0;

                let mut items = vec![(
                    "first".to_owned(),
                    "Use the first asset everytime".to_owned(),
                )];

                if let Some(asset) = file_opt {
                    idx = 1;
                    items.push((asset.to_owned(), format!("From URL: {asset}")));

                    if asset.contains(&tag) && asset != tag {
                        let t = asset.replace(&tag, "");
                        items.push((t.clone(), format!("without tag name: {t}")));
                    };
                };

                items.push((
                    String::new(),
                    if let Some(f) = file_opt {
                        format!("Edit '{f}'")
                    } else {
                        "Set asset manually".to_owned()
                    },
                ));

                let str_list: Vec<String> = items.iter().map(|t| t.1.clone()).collect();

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Which asset to use?")
                    .items(&str_list)
                    .default(idx)
                    .interact_opt()?
                    .ok_or(anyhow!("Cancelled"))?;

                let asset = match items[selection].0.as_str() {
                    "" => {
                        let inferred = file_opt.unwrap_or("");

                        let input: String = Input::new()
                            .with_prompt("Asset name?")
                            .with_initial_text(inferred)
                            .default(inferred.into())
                            .interact_text()?;

                        input
                    }

                    a => a.to_owned(),
                };

                Ok(Self::GithubRelease { repo, tag, asset })
            }

            Some(_) | None => {
                let items = vec!["Add as Custom URL", "Add as Jenkins", "Nevermind, cancel"];
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

                        Ok(Self::Url {
                            url: urlstr.to_owned(),
                            filename: Some(input),
                        })
                    }
                    Some(1) => {
                        // TODO: make it better
                        println!(" >>> {}", url.domain().unwrap());
                        let j_url = if Confirm::new()
                            .with_prompt("Is this the correct jenkins server url?")
                            .interact()?
                        {
                            url.domain().unwrap().to_owned()
                        } else {
                            Input::<String>::new()
                                .with_prompt("Jenkins URL:")
                                .with_initial_text(urlstr)
                                .default(urlstr.into())
                                .interact_text()?
                        };

                        let job: String = Input::new().with_prompt("Job:").interact_text()?;

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
                    }
                    None | Some(_) => bail!("Cancelled"),
                }
            }
        }
    }
}

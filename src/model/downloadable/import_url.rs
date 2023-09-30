use anyhow::{anyhow, bail, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use reqwest::Url;

use crate::{
    sources::modrinth::ModrinthVersion, App,
};

use super::Downloadable;

impl Downloadable {
    #[allow(clippy::too_many_lines)]
    pub async fn from_url_interactive(
        app: &App,
        urlstr: &str,
        datapack_mode: bool,
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
                    || !["mod", "plugin", "datapack"].contains(segments.first().unwrap())
                {
                    Err(invalid_url("must start with /mod, /plugin or /datapack"))?;
                };

                let id = segments
                    .get(1)
                    .ok_or_else(|| invalid_url("no id"))?
                    .to_owned()
                    .to_owned();

                let versions: Vec<ModrinthVersion> = app.modrinth().fetch_versions(&id)
                    .await?
                    .into_iter()
                    .filter(|v| {
                        if datapack_mode {
                            v.loaders.contains(&"datapack".to_owned())
                        } else {
                            !v.loaders.contains(&"datapack".to_owned())
                        }
                    })
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
                        .with_prompt("  Which version?")
                        .default(0)
                        .items(
                            &versions
                                .iter()
                                .map(|v| {
                                    let num = &v.version_number;
                                    let name = &v.name;
                                    let compat = v.loaders.join(",");
                                    let vers = v.game_versions.join(",");
                                    format!("[{num}]: {name} ({compat} ; {vers})")
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
            Some("curserinth.kuylar.dev") => {
                let invalid_url = |r| anyhow!("Invalid curserinth project URL: {r}");

                let segments: Vec<&str> = url
                    .path_segments()
                    .ok_or_else(|| invalid_url("couldn't split to segments"))?
                    .collect();

                if segments.first().is_none() || *segments.first().unwrap() != "mod" {
                    Err(invalid_url("must start with /mod"))?;
                };

                let id = segments
                    .get(1)
                    .ok_or_else(|| invalid_url("must include project id"))?
                    .to_owned()
                    .to_owned();

                let (versions, _) = app.curserinth().fetch_versions(&id).await?;

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
                        bail!("No compatible versions in curserinth project");
                    }

                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("  Which version?")
                        .default(0)
                        .items(
                            &versions
                                .iter()
                                .map(|v| {
                                    let num = &v.version_number;
                                    let name = &v.name;
                                    let compat = v.loaders.join(",");
                                    let vers = v.game_versions.join(",");
                                    format!("[{num}]: {name} ({compat} ; {vers})")
                                })
                                .collect::<Vec<String>>(),
                        )
                        .interact()
                        .unwrap();

                    versions[selection].clone()
                };

                Ok(Self::CurseRinth {
                    id,
                    version: version.id,
                })
            }
            Some("www.curseforge.com") => {
                // https://www.curseforge.com/minecraft/mc-mods/betterwithpatches
                let segments: Vec<&str> = url
                    .path_segments()
                    .ok_or_else(|| anyhow!("Invalid url"))?
                    .collect();

                if segments.first().is_none()
                    || *segments.first().unwrap() != "minecraft"
                    || segments.get(1).is_none()
                    || *segments.get(1).unwrap() != "mc-mods"
                {
                    Err(anyhow!(
                        "Invalid Curseforge URL - should start with /minecraft/mc-mods"
                    ))?;
                }

                let id = "mod__".to_owned()
                    + segments
                        .get(2)
                        .ok_or_else(|| anyhow!("Invalid Curseforge URL - mod id not found in URL"))?
                        .to_owned();

                let version = if let Some(v) = segments.get(4) {
                    v.to_owned().to_owned()
                } else {
                    let (versions, _) = app.curserinth().fetch_versions(&id)
                        .await?;

                    if versions.is_empty() {
                        bail!("No compatible versions in curseforge/rinth project");
                    }

                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("  Which version?")
                        .default(0)
                        .items(
                            &versions
                                .iter()
                                .map(|v| {
                                    let num = &v.version_number;
                                    let name = &v.name;
                                    let compat = v.loaders.join(",");
                                    let vers = v.game_versions.join(",");
                                    format!("[{num}]: {name} ({compat} ; {vers})")
                                })
                                .collect::<Vec<String>>(),
                        )
                        .interact()
                        .unwrap();

                    versions[selection].clone().id
                };

                Ok(Downloadable::CurseRinth { id, version })
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
                    version: "latest".to_owned()
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

                            println!("  > Implied release: {tag}");
                        }
                        Some("download") => {
                            let invalid_url = || anyhow!("Invalid github release download url");

                            let tag = segments.next().ok_or_else(invalid_url)?;

                            tag_opt = Some(tag.to_owned());

                            println!("  > Implied release: '{tag}'");

                            let file = segments.next().ok_or_else(invalid_url)?;

                            file_opt = Some(file);

                            println!("  > Implied asset: '{tag}'");
                        }
                        Some(p) => bail!("No idea what to do with releases/{p}"),
                        None => {}
                    }
                };

                let fetched_tags = app.github().fetch_releases(&repo).await?;

                let tag = if let Some(t) = tag_opt {
                    t
                } else {
                    let mut items = vec!["Always use latest release".to_owned()];

                    for r in &fetched_tags {
                        items.push(format!("Release {}", r.name));
                    }

                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("  Which release to use?")
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
                    items.push((asset.to_owned(), asset.to_owned()));

                    if asset.contains(&tag) && asset != tag {
                        let t = asset.replace(&tag, "${tag}");
                        items.push((t.clone(), t));
                        idx = 2;
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
                    .with_prompt("  Which asset to use?")
                    .items(&str_list)
                    .default(idx)
                    .interact_opt()?
                    .ok_or(anyhow!("Cancelled"))?;

                let asset = match items[selection].0.as_str() {
                    "" => {
                        let inferred = file_opt.unwrap_or("");

                        let input: String = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("  Asset name?")
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
                let items = vec!["Add as Custom URL", "Add as Jenkins"];
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("  How would you like to import this URL?\n  -> {urlstr}"))
                    .items(&items)
                    .default(if url.path().starts_with("/job") {
                        1
                    } else {
                        0
                    })
                    .interact()?;

                match selection {
                    0 => {
                        let inferred = urlstr
                            .split('?')
                            .next()
                            .unwrap_or(urlstr)
                            .split('/')
                            .last()
                            .unwrap();

                        let input: String = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("  Filename?")
                            .with_initial_text(inferred)
                            .default(inferred.into())
                            .interact_text()?;

                        let desc: String = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("  Optional description/comment?")
                            .default(String::new())
                            .interact_text()?;

                        Ok(Self::Url {
                            url: urlstr.to_owned(),
                            filename: Some(input),
                            desc: if desc.is_empty() { None } else { Some(desc) },
                        })
                    }
                    1 => {
                        // TODO: make it better..?
                        let j_url = if Confirm::with_theme(&ColorfulTheme::default())
                            .with_prompt(
                                "  Is this the correct jenkins server url?\n  > https://"
                                    .to_owned()
                                    + url.domain().unwrap(),
                            )
                            .interact()?
                        {
                            "https://".to_owned() + url.domain().unwrap()
                        } else {
                            Input::<String>::with_theme(&ColorfulTheme::default())
                                .with_prompt("  Jenkins URL:")
                                .with_initial_text(urlstr)
                                .default(urlstr.into())
                                .interact_text()?
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

                        let job: String = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("  Jenkins Job:")
                            .with_initial_text(&inferred_job)
                            .default(inferred_job)
                            .interact_text()?;

                        let build: String = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("  Jenkins Build:")
                            .with_initial_text("latest")
                            .default("latest".into())
                            .interact_text()?;

                        let artifact: String = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("  Jenkins Artifact:")
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
                    _ => unreachable!(),
                }
            }
        }
    }
}

use anyhow::{bail, Context, Result};
use std::borrow::Cow;

use crate::{
    app::{App, Prefix},
    model::Downloadable,
    util::SelectItem,
};

#[derive(clap::Args)]
pub struct Args {
    search: Option<String>,
}

pub async fn run(mut app: App, args: Args) -> Result<()> {
    let search_type = app.select(
        "Which project type?",
        &[
            SelectItem("mod", Cow::Borrowed("Mods")),
            SelectItem("datapack", Cow::Borrowed("Datapacks")),
            SelectItem("modpack", Cow::Borrowed("Modpacks")),
        ],
    )?;

    let query = if let Some(s) = args.search {
        s.clone()
    } else {
        app.prompt_string("Search on Modrinth")?
    };

    let projects = app
        .modrinth()
        .search(&query)
        .await
        .context("Searching modrinth")?;

    if projects.is_empty() {
        bail!("No modrinth projects found for query '{query}'");
    }

    let items = projects
        .into_iter()
        .filter(|p| p.project_type == search_type)
        .map(|p| {
            SelectItem(
                p.clone(),
                Cow::Owned(format!(
                    "{} [{}]\n{s:w$}{}",
                    p.title,
                    p.slug,
                    p.description,
                    s = " ",
                    w = 4
                )),
            )
        })
        .collect::<Vec<_>>();

    let project = app.select("Which project?", &items)?;

    let versions = app
        .modrinth()
        .fetch_versions(&project.slug)
        .await
        .context("Fetching modrinth versions")?;

    let version = app.select(
        "Which version?",
        &versions
            .into_iter()
            .map(|v| {
                SelectItem(
                    v.clone(),
                    Cow::Owned(format!("[{}]: {}", v.version_number, v.name)),
                )
            })
            .collect::<Vec<_>>(),
    )?;

    match if version.loaders.iter().any(|s| s.as_str() == "datapack") {
        if version.loaders.len() > 1 {
            app.select(
                "Import as...",
                &[
                    SelectItem("datapack", Cow::Borrowed("Datapack")),
                    SelectItem("mod", Cow::Borrowed("Mod/Plugin")),
                ],
            )?
        } else {
            "datapack"
        }
    } else {
        project.project_type.as_str()
    } {
        "modpack" => {
            todo!("Modpack importing currently unsupported")
        }
        "mod" => {
            app.add_addon_inferred(Downloadable::Modrinth {
                id: project.slug.clone(),
                version: version.id.clone(),
            })?;

            app.save_changes()?;
            app.notify(Prefix::Imported, format!("{} from modrinth", project.title));
            app.refresh_markdown().await?;
        }
        "datapack" => {
            app.add_datapack(Downloadable::Modrinth {
                id: project.slug.clone(),
                version: version.id.clone(),
            })?;

            app.save_changes()?;
            app.refresh_markdown().await?;
        }
        ty => bail!("Unsupported modrinth project type: '{ty}'"),
    }

    Ok(())
}

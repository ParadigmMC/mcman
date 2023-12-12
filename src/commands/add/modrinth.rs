use anyhow::{bail, Context, Result};

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
            SelectItem("mod", "Mods".to_owned()),
            SelectItem("datapack", "Datapacks".to_owned()),
            SelectItem("modpack", "Modpacks".to_owned()),
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
            let str = format!(
                "{} [{}]\n{s:w$}{}",
                p.title,
                p.slug,
                p.description,
                s = " ",
                w = 4
            );

            SelectItem(p, str)
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
                let str = format!("[{}]: {}", v.version_number, v.name,);
                SelectItem(v, str)
            })
            .collect::<Vec<_>>(),
    )?;

    match if version.loaders.contains(&"datapack".to_owned()) {
        if version.loaders.len() > 1 {
            app.select(
                "Import as...",
                &[
                    SelectItem("datapack", "Datapack".to_owned()),
                    SelectItem("mod", "Mod/Plugin".to_owned()),
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
            let addon = Downloadable::Modrinth {
                id: project.slug.clone(),
                version: version.id.clone(),
            };

            app.add_addon_inferred(&addon)?;

            app.save_changes()?;
            app.notify(Prefix::Imported, format!("{} from modrinth", project.title));
            app.refresh_markdown().await?;
        }
        "datapack" => {
            let dp = Downloadable::Modrinth {
                id: project.slug.clone(),
                version: version.id.clone(),
            };

            app.add_datapack(&dp)?;

            app.save_changes()?;
            app.refresh_markdown().await?;
        }
        ty => bail!("Unsupported modrinth project type: '{ty}'"),
    }

    Ok(())
}

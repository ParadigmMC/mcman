use anyhow::{bail, Result};

use crate::{
    model::Downloadable,
    util::SelectItem, app::{App, Prefix},
};

#[derive(clap::Args)]
pub struct Args {
    search: Option<String>,
}

pub async fn run(mut app: App, args: Args) -> Result<()> {
    let search_type = app.select("Which project type?", &[
        SelectItem("mod", "Mods".to_owned()),
        SelectItem("datapack", "Datapacks".to_owned()),
        SelectItem("modpack", "Modpacks".to_owned()),
    ])?;

    let query = if let Some(s) = args.search {
        s.to_owned()
    } else {
        app.prompt_string("Search on Modrinth")?
    };

    let projects = app.modrinth().search(&query).await?;

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
                w = 10
            );

            SelectItem(
                p,
                str,
            )
        })
        .collect::<Vec<_>>();

    let project = app.select("Which project?", &items)?;

    let versions = app.modrinth().fetch_versions(&project.slug).await?;

    let version = app.select("Which version?", &versions
        .into_iter()
        .map(|v| {
            let str = format!(
                "[{}]: {}",
                v.version_number,
                v.name,
            );
            SelectItem(v, str)
        }).collect::<Vec<_>>())?;

    match if version.loaders.contains(&"datapack".to_owned()) {
        if version.loaders.len() > 1 {
            app.select("Import as...", &[
                SelectItem("datapack", "Datapack".to_owned()),
                SelectItem("mod", "Mod/Plugin".to_owned()),
            ])?
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
            app.refresh_markdown().await?;

            app.notify(Prefix::Imported, format!("{} from modrinth", project.title));
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

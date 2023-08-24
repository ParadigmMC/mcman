use anyhow::{Context, Result, bail};
use clap::{arg, ArgMatches, Command};
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::{
    create_http_client,
    model::{Downloadable, Server, SoftwareType}, sources::modrinth, util::SelectItem,
};

pub fn cli() -> Command {
    Command::new("modrinth")
        .about("Add from modrinth")
        .visible_alias("mr")
        .arg(arg!(<search>...).required(false))
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let query = if let Some(s) = matches.get_one::<String>("search") {
        s.to_owned()
    } else {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Search on modrinth:")
            .allow_empty(true)
            .interact_text()?
    };

    let facets = server.jar.get_modrinth_facets(&server.mc_version)?;

    let projects = modrinth::search_modrinth(&http_client, &query, &facets).await?;

    if projects.is_empty() {
        bail!("No modrinth projects found for query '{query}'");
    }

    let items = projects.iter().map(|p| {
        SelectItem(p, format!("{} {} [{}]\n{s:w$}{}", match p.project_type.as_str() {
            "mod" => "(mod)",
            "datapack" => "( dp)",
            "modpack" => "(mrp)",
            _ => "( ? )",
        }, p.title, p.slug, p.description, s = " ", w = 10))
    }).collect::<Vec<_>>();

    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which project?")
        .items(&items)
        .default(0)
        .interact()?;

    let project = items[idx].0.clone();

    let versions = modrinth::fetch_modrinth_versions(&http_client, &project.slug, None).await?;

    let versions = server.filter_modrinth_versions(&versions);

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

    let version = versions[selection].clone();

    match if version.loaders.contains(&"datapack".to_owned()) {
        if version.loaders.len() > 1 {
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Import as...")
                .default(0)
                .items(&["Datapack", "Mod/Plugin"])
                .interact()? {
                0 => "datapack",
                1 => "mod",
                _ => unreachable!(),
            }
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
            let addon = Downloadable::Modrinth { id: project.slug.clone(), version: version.id.clone() };

            let is_plugin = match server.jar.get_software_type() {
                SoftwareType::Modded => false,
                SoftwareType::Normal | SoftwareType::Proxy => true,
                SoftwareType::Unknown => {
                    Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Import as...")
                        .default(0)
                        .items(&["Plugin", "Mod"])
                        .interact()?
                        == 0
                }
            };
        
            if is_plugin {
                server.plugins.push(addon);
            } else {
                server.mods.push(addon);
            }
        
            server.save()?;
        
            server.refresh_markdown(&http_client).await?;
        
            println!(" > Added {} from modrinth", project.title);
        }
        "datapack" => {
            let addon = Downloadable::Modrinth { id: project.slug.clone(), version: version.id.clone() };

            let world_name = server.add_datapack(addon)?;
        
            server.save()?;

            server.refresh_markdown(&http_client).await?;
        
            println!(
                " > {} {} {} {world_name}{}",
                style("Datapack ").green(),
                project.title,
                style("added to").green(),
                style("!").green()
            );
        }
        ty => bail!("Unsupported modrinth project type: '{ty}'"),
    }

    Ok(())
}

use anyhow::{bail, Result};
use dialoguer::{theme::ColorfulTheme, Select};

use crate::util::SelectItem;

use super::Downloadable;

impl Downloadable {
    pub fn select_jar_interactive() -> Result<Self> {
        let items = vec![
            SelectItem(0, "Vanilla       - No patches".to_owned()),
            SelectItem(1, "PaperMC/Paper - Spigot fork, most popular".to_owned()),
            SelectItem(2, "Purpur        - Paper fork".to_owned()),
            SelectItem(
                3,
                "BuildTools    - Spigot, Bukkit or CraftBukkit".to_owned(),
            ),
        ];

        let jar_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which server software to use?")
            .default(0)
            .items(&items)
            .interact()?;

        Ok(match jar_type {
            0 => Self::Vanilla {},
            1 => Self::Paper {},
            2 => Self::Purpur {
                build: "latest".to_owned(),
            },
            3 => {
                let items = vec![
                    SelectItem(Self::BuildTools { args: vec![] }, "Spigot".to_owned()),
                    SelectItem(
                        Self::BuildTools {
                            args: vec!["--compile".to_owned(), "craftbukkit".to_owned()],
                        },
                        "CraftBukkit".to_owned(),
                    ),
                ];

                let idx = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Which one?")
                    .default(0)
                    .items(&items)
                    .interact()?;

                items[idx].0.clone()
            }
            _ => unreachable!(),
        })
    }

    pub fn select_modded_jar_interactive() -> Result<Self> {
        let items = [
            (0, "Quilt  - Modern, fabric compatible (Beta)"),
            (1, "Fabric - Lightweight"),
            //(2, "Forge  - Ye' olde modde"),
        ];

        let items_str: Vec<String> = items.iter().map(|v| v.1.to_owned()).collect();

        let jar_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which (modded) server software to use?")
            .default(0)
            .items(&items_str)
            .interact()?;

        Ok(match jar_type {
            0 => Self::Quilt {
                loader: "latest".to_owned(),
                installer: "latest".to_owned(),
            },
            1 => Self::Fabric {
                loader: "latest".to_owned(),
                installer: "latest".to_owned(),
            },
            2 => todo!(),
            _ => unreachable!(),
        })
    }

    pub fn select_proxy_jar_interactive() -> Result<Self> {
        let items = [
            (0, "Velocity   - Modern, high perf. proxy by PaperMC"),
            (1, "Waterfall  - BungeeCord fork by PaperMC"),
            (2, "Bungeecord - By md5 (Spigot)"),
        ];

        let items_str: Vec<String> = items.iter().map(|v| v.1.to_owned()).collect();

        let jar_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which proxy software to use?")
            .default(0)
            .items(&items_str)
            .interact()?;

        Ok(match jar_type {
            0 => Self::Velocity {},
            1 => Self::Waterfall {},
            2 => Self::BungeeCord {},
            3 => {
                bail!("Please add the custom proxy jar via manually editing the server.toml file.");
            }
            _ => unreachable!(),
        })
    }
}

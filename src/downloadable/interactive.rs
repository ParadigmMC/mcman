use anyhow::{Result, bail};
use dialoguer::{Select, theme::ColorfulTheme};

use super::Downloadable;

impl Downloadable {
    pub fn select_jar_interactive() -> Result<Self> {
        let items = vec![
            (0, "Vanilla       - No patches"),
            (1, "PaperMC/Paper - Spigot fork, most popular"),
            (2, "Purpur        - Paper fork"),
        ];

        let items_str: Vec<String> = items.iter().map(|v| v.1.to_owned()).collect();

        let jar_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which server software to use?")
            .default(0)
            .items(&items_str)
            .interact()?;

        Ok(match jar_type {
            0 => Self::Vanilla {  },
            1 => Self::Paper {  },
            2 => Self::Purpur { build: "latest".to_owned() },
            _ => unreachable!(),
        })
    }

    pub fn select_modded_jar_interactive() -> Result<Self> {
        let items = vec![
            (0, "Quilt  - Modern, fabric compatible (Beta)"),
            (1, "Fabric - Lightweight"),
            //(2, "Forge  - Ye' olde modde"),
        ];

        let items_str: Vec<String> = items.iter().map(|v| v.1.to_owned()).collect();

        let jar_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which server software to use?")
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
        let items = vec![
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
            0 => Self::Velocity {  },
            1 => Self::Waterfall {  },
            2 => Self::BungeeCord {  },
            3 => {
                bail!("Please add the custom proxy jar via manually editing the server.toml file.");
            },
            _ => unreachable!(),
        })
    }
}
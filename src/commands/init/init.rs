use anyhow::{Result, Context};
use dialoguer::{Input, theme::ColorfulTheme, Select};

use crate::{App, model::{ServerLauncher, ServerType}};

pub async fn init_normal(app: &mut App) -> Result<()> {
    let serv_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Type of server?")
        .default(0)
        .items(&[
            "Normal Server (vanilla, spigot, paper etc.)",
            "Modded Server (forge, fabric, quilt etc.)",
            "Proxy Server (velocity, bungeecord, waterfall etc.)",
        ])
        .interact()?;

    let is_proxy = serv_type == 2;

    app.server.mc_version = if is_proxy {
        "latest".to_owned()
    } else {
        let latest_ver = app.vanilla().fetch_latest_mcver()
            .await
            .context("Fetching latest version")?;

        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Server version?")
            .default(latest_ver)
            .interact_text()?
    };

    app.server.launcher = if is_proxy {
        ServerLauncher {
            proxy_flags: true,
            aikars_flags: false,
            nogui: false,
            ..Default::default()
        }
    } else {
        ServerLauncher::default()
    };

    app.server.jar = match serv_type {
        0 => ServerType::select_jar_interactive(),
        1 => ServerType::select_modded_jar_interactive(),
        2 => ServerType::select_proxy_jar_interactive(),
        _ => unreachable!(),
    }?;

    Ok(())
}

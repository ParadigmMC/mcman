use crate::model::Server;
use crate::util::md::MarkdownTable;
use anyhow::{Context, Result};
use console::style;
use indexmap::IndexMap;

use super::markdown::create_table_server_console;

pub fn run() -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;

    let table = create_table_server_console(&server);

    let mut server_info = IndexMap::new();

    for idx in 0..table.rows[0].len() {
        let k = table.headers[idx].clone();
        let v = table.rows[0][idx].clone();

        server_info.insert(k, v);
    }

    let pad_keys = server_info
        .iter()
        .max_by_key(|(k, _)| k.len())
        .unwrap()
        .0
        .len();

    for (k, v) in &server_info {
        let k_styled = style(format!("{k:pad_keys$}")).cyan().bold();

        println!(" {k_styled}: {v}");
    }

    if !server.plugins.is_empty() {
        println!(
            " {:pad_keys$}> {} {}",
            "",
            style(server.plugins.len()).bold(),
            style("Plugins").cyan(),
        );

        let mut table = MarkdownTable::new();

        for plugin in &server.plugins {
            table.add_from_map(&plugin.fields_to_map());
        }

        let text = table.render_ascii();

        println!("{text}");
    }

    if !server.mods.is_empty() {
        println!(
            " {:pad_keys$}> {} {}",
            "",
            style(server.mods.len()).bold(),
            style("Mods").cyan(),
        );

        let mut table = MarkdownTable::new();

        for addon in &server.mods {
            table.add_from_map(&addon.fields_to_map());
        }

        let text = table.render_ascii();

        println!("{text}");
    }

    Ok(())
}

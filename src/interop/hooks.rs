use std::collections::HashMap;

use anyhow::Result;

use crate::{app::App, model::HookEvent};

pub struct HooksAPI<'a>(pub &'a App);

impl<'a> HooksAPI<'a> {
    pub fn resolve_name(&self, entry: &str) -> String {
        let hook = self.0.server.hooks.get(entry)
            .or(self.0.network.as_ref().and_then(|nw| nw.hooks.get(entry).clone()))
            .unwrap();
        
        match std::env::consts::FAMILY {
            "windows" => hook.windows.clone(),
            "unix" => hook.linux.clone(),
            _ => None,
        }.unwrap_or(String::from(entry))
    }

    pub async fn event(&self, event: HookEvent, data: HashMap<String, String>) -> Result<()> {
        for (name, hook) in &self.0.server.hooks {
            if !hook.disabled && hook.when == event {
                // execute?
            }
        }

        Ok(())
    }
}

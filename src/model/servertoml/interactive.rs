use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select, Input};

use crate::{model::{Downloadable, World}, util::SelectItem};

use super::Server;

impl Server {
    pub fn add_datapack(&mut self, dl: Downloadable) -> Result<String> {
        let selected_world_name = if self.worlds.is_empty() {
            "+".to_owned()
        } else {
            let mut items: Vec<SelectItem<String>> = self
                .worlds
                .keys()
                .map(|k| SelectItem(k.clone(), k.clone()))
                .collect();
    
            items.push(SelectItem("+".to_owned(), "+ New world entry".to_owned()));
    
            let idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which world to add to?")
                .items(&items)
                .default(items.len() - 1)
                .interact()?;
    
            items[idx].0.clone()
        };
    
        let world_name = if selected_world_name == "+" {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("World name?")
                .default("world".to_owned())
                .interact_text()?
        } else {
            selected_world_name
        };
    
        if !self.worlds.contains_key(&world_name) {
            self.worlds.insert(world_name.clone(), World::default());
        }
    
        self
            .worlds
            .get_mut(&world_name)
            .expect("world shouldve already been inserted")
            .datapacks
            .push(dl);

        Ok(world_name)
    }
}
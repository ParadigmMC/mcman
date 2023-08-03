use std::{collections::HashMap, env, io::Write};

use anyhow::Result;
use console::style;

use crate::bootstrapper::{bootstrap, BootstrapContext};

use super::BuildContext;

impl BuildContext {
    pub fn bootstrap_files(&self) -> Result<()> {
        let mut vars = HashMap::new();

        for (key, value) in &self.server.variables {
            vars.insert(key.clone(), value.clone());
        }

        for (key, value) in env::vars() {
            vars.insert(key.clone(), value.clone());
        }

        vars.insert("SERVER_NAME".to_owned(), self.server.name.clone());
        vars.insert("SERVER_VERSION".to_owned(), self.server.mc_version.clone());

        bootstrap(
            &BootstrapContext {
                vars,
                output_dir: self.output_dir.clone(),
            },
            "config",
        )?;

        if self.server.launcher.eula_args {
            if !self.server.jar.supports_eula_args() {
                println!(
                    "          {}",
                    style("=> eula.txt [eula_args unsupported]").dim()
                );
                std::fs::File::create(self.output_dir.join("eula.txt"))?
                    .write_all(b"eula=true\n")?;
            }
        }

        println!("          {}", style("Bootstrapping complete").dim());

        Ok(())
    }
}

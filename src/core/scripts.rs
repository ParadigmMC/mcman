use std::{fs::OpenOptions, io::Write};

use anyhow::Result;
use console::style;
use tokio::fs;

use super::BuildContext;

impl BuildContext {
    pub async fn create_scripts(&self) -> Result<()> {
        fs::write(
            self.output_dir.join("start.bat"),
            self.server
                .launcher
                .generate_script_win(&self.server.name, &self.startup_method),
        )
        .await?;

        let mut file;
        #[cfg(target_family = "unix")]
        {
            use std::os::unix::prelude::OpenOptionsExt;
            file = OpenOptions::new()
                .create(true)
                .write(true)
                .mode(0o755)
                .open(self.output_dir.join("start.sh"))?;
        }
        #[cfg(not(target_family = "unix"))]
        {
            file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(self.output_dir.join("start.sh"))?;
        }

        file.write_all(
            self.server
                .launcher
                .generate_script_linux(&self.server.name, &self.startup_method)
                .as_bytes(),
        )?;

        println!(
            "          {}",
            style("start.bat and start.sh created").dim()
        );

        Ok(())
    }
}

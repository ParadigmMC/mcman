use std::{fs::OpenOptions, io::Write};

use anyhow::Result;
use tokio::fs;

use crate::model::StartupMethod;

use super::BuildContext;

impl<'a> BuildContext<'a> {
    pub async fn create_scripts(&self, startup: StartupMethod) -> Result<()> {
        fs::write(
            self.output_dir.join("start.bat"),
            self.app.server
                .launcher
                .generate_script_win(&self.app.server.name, &startup),
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
            self.app.server
                .launcher
                .generate_script_linux(&self.app.server.name, &startup)
                .as_bytes(),
        )?;

        Ok(())
    }
}

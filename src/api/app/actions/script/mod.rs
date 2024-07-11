use std::path::Path;

use anyhow::Result;

use crate::api::{app::App, models::server::Server, utils::script::Shell};

impl App {
    fn get_script_lines_for(&self, shell: &Shell, server: &Server) -> Vec<String> {
        let mut lines = vec![];
    
        if *shell == Shell::Bat {
            lines.push(format!("title {}", server.name));
        }

        lines.extend(server.launcher.prelaunch.clone());
        let mut args = server.get_arguments();
        args.push(shell.script_args().to_owned());
        lines.push(args.join(" "));
        lines.extend(server.launcher.postlaunch.clone());
        lines
    }

    pub fn action_generate_script(&self, shell: Shell, server: &Server, base: &Path) -> Result<()> {
        let script = shell.generate_script(self.get_script_lines_for(&shell, server));

        std::fs::write(base.join(format!("start.{}", shell.file_ext())), script)?;

        Ok(())
    }

    pub async fn action_generate_scripts(&self, base: &Path) -> Result<()> {
        if let Some((_, server)) = &*self.server.read().await {
            self.action_generate_script(Shell::Bat, server, base)?;
            self.action_generate_script(Shell::Sh, server, base)?;
        }

        Ok(())
    }
}

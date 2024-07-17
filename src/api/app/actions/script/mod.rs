use std::path::Path;

use anyhow::Result;

use crate::api::{app::App, models::server::Server, tools::java::{get_java_installation_for, JavaInstallation}, utils::script::Shell};

impl App {
    async fn get_script_lines_for(&self, shell: &Shell, server: &Server) -> Result<Vec<String>> {
        let mut lines = vec![];
    
        if *shell == Shell::Bat {
            lines.push(format!("title {}", server.name));
        }

        lines.extend(server.launcher.prelaunch.clone());

        let mut args = vec![];

        let java = if let Some(v) = &server.launcher.java_version {
            get_java_installation_for(*v).await.map(|j| j.path.to_string_lossy().into_owned()).unwrap_or(String::from("java"))
        } else {
            String::from("java")
        };

        args.push(java);
        args.extend(server.get_arguments());
        args.push(shell.script_args().to_owned());

        lines.push(args.join(" "));

        lines.extend(server.launcher.postlaunch.clone());
        Ok(lines)
    }

    pub async fn action_generate_script(&self, shell: Shell, server: &Server, base: &Path) -> Result<()> {
        let script = shell.generate_script(self.get_script_lines_for(&shell, server).await?);

        tokio::fs::write(base.join(format!("start.{}", shell.file_ext())), script)
            .await?;

        Ok(())
    }

    pub async fn action_generate_scripts(&self, base: &Path) -> Result<()> {
        if let Some((_, server)) = &*self.server.read().await {
            self.action_generate_script(Shell::Bat, server, base).await?;
            self.action_generate_script(Shell::Sh, server, base).await?;
        }

        Ok(())
    }
}
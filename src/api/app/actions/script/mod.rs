use anyhow::Result;

use crate::api::{app::App, models::server::Server, utils::script::Shell};

impl App {
    pub fn get_server_execution_arguments(&self) -> Vec<String> {
        todo!()
    }

    pub fn get_script_lines_for(&self, shell: Shell, server: &Server) -> Vec<String> {
        let mut lines = vec![];
        
        if shell == Shell::Bat {
            lines.push(format!("title {}", server.name));
        }

        lines.extend(server.launcher.prelaunch.clone());

        todo!();

        lines.extend(server.launcher.postlaunch.clone());

        lines
    }

    pub async fn action_generate_script(&self) -> Result<()> {
        todo!()
    }
}

use anyhow::Result;

use crate::api::{app::App, models::server::Server, utils::script::Shell};

impl App {
    pub fn get_args(&self, server: &Server) -> Vec<String> {
        let (prefix, suffix) = self.get_args_prefix_suffix(server);
        let exec = self.get_args_exec(server);

        vec![prefix, exec, suffix].concat()
    }

    pub fn get_args_exec(&self, server: &Server) -> Vec<String> {
        server.jar.as_ref().map(|s| s.get_exec_arguments()).unwrap_or_default()
    }

    pub fn get_args_prefix_suffix(&self, server: &Server) -> (Vec<String>, Vec<String>) {
        let mut prefix = vec![];

        prefix.extend(server.launcher.jvm_args.split_whitespace().map(ToOwned::to_owned));

        // TODO: -Xmx -Xms

        prefix.extend(server.launcher.preset_flags.get_flags());

        if server.launcher.eula_args && server.jar.as_ref().is_some_and(|x| x.flavor().supports_eula_args()) {
            prefix.push(String::from("-Dcom.mojang.eula.agree=true"));
        }

        for (key, value) in &server.launcher.properties {
            let value = serde_json::to_string(value).unwrap();

            prefix.push(format!("-D{key}={value}"));
        }

        let mut suffix = vec![];

        if server.launcher.nogui && server.jar.as_ref().is_some_and(|x| x.flavor().supports_nogui()) {
            suffix.push(String::from("--nogui"));
        }

        suffix.extend(server.launcher.game_args.split_whitespace().map(ToOwned::to_owned));

        (prefix, suffix)
    }

    pub fn get_script_lines_for(&self, shell: Shell, server: &Server) -> Vec<String> {
        let mut lines = vec![];
        
        if shell == Shell::Bat {
            lines.push(format!("title {}", server.name));
        }

        lines.extend(server.launcher.prelaunch.clone());

        lines.push(self.get_args(server).join(" "));

        lines.extend(server.launcher.postlaunch.clone());

        lines
    }

    pub async fn action_generate_script(&self) -> Result<()> {
        todo!()
    }
}

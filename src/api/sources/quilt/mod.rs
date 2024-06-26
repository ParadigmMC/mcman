use anyhow::{bail, Result};

use crate::api::{app::App, models::Environment, step::Step};

pub struct QuiltAPI<'a>(pub &'a App);

pub const QUILT_MAVEN_URL: &str = "https://maven.quiltmc.org/repository/release";
pub const QUILT_MAVEN_GROUP: &str = "org.quiltmc";
pub const QUILT_MAVEN_ARTIFACT: &str = "quilt-installer";
pub const QUILT_MAVEN_FILE: &str = "${artifact}-${version}.jar";

impl<'a> QuiltAPI<'a> {
    pub async fn resolve_steps(
        &self,
        mc_version: &str,
        quilt_installer: &str,
        quilt_loader: &str,
        env: &Environment,
    ) -> Result<Vec<Step>> {
        let mut steps: Vec<Step> = vec![];

        let installer = self.resolve_steps_jar(quilt_installer).await?;

        let jar_name = installer.iter()
            .find_map(|s| {
                if let Step::CacheCheck(m) = s { Some(m) } else { None }
            })
            .unwrap()
            .filename.clone();

        steps.extend(installer);

        if env.server() {
            steps.extend(self.resolve_steps_build(&jar_name, mc_version, quilt_loader, &Environment::Server).await?);
        }

        if env.client() {
            steps.extend(self.resolve_steps_build(&jar_name, mc_version, quilt_loader, &Environment::Client).await?);
        }

        Ok(steps)
    }

    /// Resolve steps to downloading quilt installer
    pub async fn resolve_steps_jar(&self, quilt_installer: &str) -> Result<Vec<Step>> {
        self.0
            .maven()
            .resolve_steps(
                QUILT_MAVEN_URL,
                QUILT_MAVEN_GROUP,
                QUILT_MAVEN_ARTIFACT,
                quilt_installer,
                QUILT_MAVEN_FILE,
            )
            .await
    }

    /// Resolve steps to executing quilt installer
    pub async fn resolve_steps_build(
        &self,
        jar_name: &str,
        mc_version: &str,
        quilt_loader: &str,
        env: &Environment,
    ) -> Result<Vec<Step>> {
        let mut args = vec![
            String::from("-jar"),
            jar_name.to_owned(),
            String::from("install"),
            env.to_string(),
            mc_version.to_owned(),
            quilt_loader.to_owned(),
            String::from("--install-dir=."),
        ];

        match env {
            Environment::Server => {
                args.push(String::from("--download-server"));
            }
            Environment::Client => {
                args.push(String::from("--no-profile"));
            }
            _ => {},
        }

        Ok(vec![
            Step::ExecuteJava {
                args,
                java_version: None,
                label: String::from("QuiltInstaller"),
            }
        ])
    }
}

use anyhow::{anyhow, Result};

use crate::api::{app::App, step::Step, tools::java::JavaVersion};

pub const BUILDTOOLS_JENKINS_URL: &str = "https://hub.spigotmc.org/jenkins";
pub const BUILDTOOLS_JENKINS_JOB: &str = "BuildTools";
pub const BUILDTOOLS_JENKINS_ARTIFACT: &str = "BuildTools.jar";

pub async fn resolve_steps(
    app: &App,
    craftbukkit: bool,
    custom_args: &Vec<String>,
    mc_version: &str,
) -> Result<Vec<Step>> {
    let jar = resolve_steps_jar(app).await?;

    let meta = jar.iter().find_map(|v| if let Step::CacheCheck(meta) = v { Some(meta) } else { None }).unwrap();

    let exec = resolve_steps_build(app, &meta.filename, craftbukkit, custom_args, mc_version).await?;

    Ok(vec![jar, exec].concat())
}

/// Resolve steps for the BuildTools.jar
pub async fn resolve_steps_jar(
    app: &App,
) -> Result<Vec<Step>> {
    app.jenkins()
        .resolve_steps(
            BUILDTOOLS_JENKINS_URL,
            BUILDTOOLS_JENKINS_JOB,
            "latest",
            BUILDTOOLS_JENKINS_ARTIFACT,
            Some("BuildTools-${build}.jar".to_owned())
        )
        .await
}

/// Resolve steps for using BuildTools to compile a server jar
pub async fn resolve_steps_build(
    _app: &App,
    jar_name: &str,
    craftbukkit: bool,
    custom_args: &Vec<String>,
    mc_version: &str,
) -> Result<Vec<Step>> {
    let mut args = vec![
        String::from("-jar"),
        jar_name.to_owned(),
        String::from("--compile-if-changed"),
        String::from("--rev"),
        mc_version.to_owned(),
        String::from("--final-name"),
        String::from("server.jar"),
    ];    

    if craftbukkit {
        args.push(String::from("--compile"));
        args.push(String::from("craftbukkit"));
    }

    args.extend(custom_args.clone());

    //let build_number = jar_name.split(&['-', '.']).nth(1).unwrap().parse::<i32>()?;

    Ok(vec![
        /* Step::CacheCheck(FileMeta {
            cache: Some(CacheLocation("buildtools".into(), format!(
                "",
            ))),
            ..Default::default()
        }), */
        Step::ExecuteJava {
            args,
            java_version: Some(get_java_version_for(mc_version)?),
            label: "BuildTools".to_owned(),
        }
    ])
}

/// Get java version to use for a minecraft version
/// ... => Java 8
/// 1.17 => Java 16
/// 1.18+ => Java 17
pub fn get_java_version_for(mc_version: &str) -> Result<JavaVersion> {
    let mut split = mc_version.split('.');
    split.next().ok_or(anyhow!("Error parsing mc_version"))?;

    match split.next().ok_or(anyhow!("Error parsing mc_version"))?.parse::<i32>()? {
        ..=16 => Ok(8),
        17 => Ok(16),
        _ => Ok(17),
    }
}

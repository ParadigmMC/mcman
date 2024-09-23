use std::{collections::HashMap, path::Path, process::Command};

use super::{JavaInstallation, JAVA_BIN};

pub fn check_java(path: &Path) -> Option<JavaInstallation> {
    let Ok(path) = std::fs::canonicalize(path) else {
        return None;
    };

    let path = if path.file_name()?.to_str()? != JAVA_BIN {
        path.join(JAVA_BIN)
    } else {
        path.clone()
    };

    if !path.exists() {
        return None;
    };

    let tempdir = tempfile::tempdir().ok()?.into_path();
    let file_path = tempdir.join("JavaInfo.class");
    std::fs::write(
        &file_path,
        include_bytes!("../../../../res/java/JavaInfo.class"),
    )
    .ok()?;

    let output = Command::new(&path)
        .arg("-cp")
        .arg(file_path.parent().unwrap())
        .arg("JavaInfo")
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut info = HashMap::new();

    for line in stdout.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        info.insert(key.to_owned(), value.to_owned());
    }

    Some(JavaInstallation {
        path,
        version: JavaInstallation::parse_version(info.get("java.version")?).ok()?,
        architecture: info.get("os.arch")?.clone(),
        vendor: info.get("java.vendor")?.clone(),
    })
}

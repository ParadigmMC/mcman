use std::{collections::HashMap, path::Path, process::Command};

use super::{extract_java_majorminor_version, JavaInstallation, JAVA_BIN};

pub fn check_java(path: &Path) -> Option<JavaInstallation> {
    let Ok(path) = std::fs::canonicalize(path) else {
        return None;
    };

    let java = if path.file_name()?.to_str()? != JAVA_BIN {
        path.join(JAVA_BIN)
    } else {
        path.clone()
    };

    if !java.exists() {
        return None;
    };

    let tempdir = tempfile::tempdir().ok()?.into_path();
    let file_path = tempdir.join("JavaInfo.class");
    std::fs::write(&file_path, include_bytes!("../../../../res/java/JavaInfo.class")).ok()?;

    let output = Command::new(&java)
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
        version: extract_java_majorminor_version(info.get("java.version")?).ok()?.1,
        architecture: info.get("os.arch")?.clone(),
        vendor: info.get("java.vendor")?.clone(),
    })
}

use std::path::PathBuf;

use anyhow::{anyhow, Result};

use super::JavaVersion;

pub struct JavaInstallation {
    pub path: PathBuf,
    pub version: JavaVersion,
    pub architecture: String,
    pub vendor: String,
}

/// Extract major/minor version from a java version string
/// Gets the minor version or an error, and assumes 1 for major version if it could not find
/// "1.8.0_361" -> (1, 8)
/// "20" -> (1, 20)
pub fn extract_java_majorminor_version(
    version: &str,
) -> Result<(u32, u32)> {
    let mut split = version.split('.');
    let major_opt = split.next();

    let mut major;
    // Try minor. If doesn't exist, in format like "20" so use major
    let mut minor = if let Some(minor) = split.next() {
        major = major_opt.unwrap_or("1").parse::<u32>()?;
        minor.parse::<u32>()?
    } else {
        // Formatted like "20", only one value means that is minor version
        major = 1;
        major_opt
            .ok_or_else(|| anyhow!("Invalid JRE version"))?
            .parse::<u32>()?
    };

    // Java start should always be 1. If more than 1, it is formatted like "17.0.1.2" and starts with minor version
    if major > 1 {
        minor = major;
        major = 1;
    }

    Ok((major, minor))
}

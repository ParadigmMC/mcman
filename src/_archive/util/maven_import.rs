use anyhow::{anyhow, bail, Result};

use crate::model::Downloadable;

macro_rules! dependency {
    ($xml:ident, $name:literal) => {
        $xml.descendants()
            .find(|t| t.tag_name().name() == $name)
            .ok_or(anyhow!(concat!("dependency.", $name, " must be present")))?
            .text()
            .ok_or(anyhow!(concat!("dependency.", $name, " must be text")))?
            .to_owned()
    };
}

/// Example:
/// ```xml
/// <dependency>
///  <groupId>net.neoforged</groupId>
///  <artifactId>forge</artifactId>
///  <version>1.20.1-47.1.62</version>
/// </dependency>
/// ```
#[allow(unused)]
pub fn import_from_maven_dependency_xml(url: &str, xml: &str) -> Result<Downloadable> {
    let xml = roxmltree::Document::parse(xml)?;

    let group = dependency!(xml, "groupId");
    let artifact = dependency!(xml, "artifactId");
    let version = dependency!(xml, "version");

    Ok(Downloadable::Maven {
        url: url.to_owned(),
        artifact,
        group,
        version,
        filename: "${artifact}-${version}".to_owned(),
    })
}

/// Gradle Kotlin:
/// ```
/// implementation("net.neoforged:forge:1.20.1-47.1.62")
/// ```
///
/// Gradle Groovy:
///
/// ```
/// implementation "net.neoforged:forge:1.20.1-47.1.62"
/// ```
#[allow(unused)]
pub fn import_from_gradle_dependency(url: &str, imp: &str) -> Result<Downloadable> {
    let imp = imp
        .replace("implementation", "")
        .replace([' ', '(', ')', '"'], "");
    let li = imp.trim().split(':').collect::<Vec<_>>();

    if li.len() != 3 {
        bail!("Gradle dependency should have 3 sections delimeted by ':' inside the quoted string");
    }

    Ok(Downloadable::Maven {
        url: url.to_owned(),
        group: li[0].to_owned(),
        artifact: li[1].to_owned(),
        version: li[2].to_owned(),
        filename: "${artifact}-${version}".to_owned(),
    })
}

/// Example:
/// ```
/// "net.neoforged" %% "forge" %% "1.20.1-47.1.62"
/// ```
#[allow(unused)]
pub fn import_from_sbt(url: &str, sbt: &str) -> Result<Downloadable> {
    let sbt = sbt.replace(char::is_whitespace, "").replace('"', "");
    let li = sbt.split('%').filter(|x| !x.is_empty()).collect::<Vec<_>>();

    if li.len() != 3 {
        bail!("SBT should have 3 sections delimeted by '%' or '%%'");
    }

    Ok(Downloadable::Maven {
        url: url.to_owned(),
        group: li[0].to_owned(),
        artifact: li[1].to_owned(),
        version: li[2].to_owned(),
        filename: "${artifact}-${version}".to_owned(),
    })
}

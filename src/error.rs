use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("server.toml already exists")]
    AlreadyExists,
    #[error("can't get package name from current path; please specify a name using --name")]
    MissingServerName,
    #[error(transparent)]
    Indicatif(#[from] indicatif::style::TemplateError),
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("mojang: can't find the server jar for version '{0}'")]
    VanillaVersionNotFound(String),

    #[error("modrinth: release '{0}' for project '{1}' not found")]
    ModrinthReleaseNotFound(String, String),

    #[error("paper: project '{0}' not found")]
    PaperMCProjectNotFound(String),
    #[error("paper: version '{1}' for project '{0}' not found")]
    PaperMCVersionNotFound(String, String),
    #[error("paper: build '{2}' for '{0}' '{1}' not found")]
    PaperMCBuildNotFound(String, String, String),
}

#[derive(Debug, Error)]
pub enum BootstrapError {
    //#[error(transparent)]
    //PropertiesError(#[from] java_properties::PropertiesError),
    //#[error(transparent)]
    //UTF8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Cli(#[from] CliError),
    #[error(transparent)]
    Fetch(#[from] FetchError),
    #[error(transparent)]
    Bootstrap(#[from] BootstrapError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSerialize(#[from] toml::ser::Error),
    #[error(transparent)]
    PropertiesError(#[from] java_properties::PropertiesError),
    #[error(transparent)]
    UTF8Error(#[from] std::string::FromUtf8Error),
}

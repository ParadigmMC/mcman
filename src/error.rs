use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("server.toml already exists")]
    AlreadyExists,
    #[error("can't get package name from current path; please specify a name using --name")]
    MissingServerName,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Cli(#[from] CliError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("can't find the server jar for version {0}")]
    VanillaVersionNotFound(String),
    
    #[error("modrinth release {0} for project {1} not found")]
    ModrinthReleaseNotFound(String, String),

    #[error("papermc.io: project {0} not found")]
    PaperMCProjectNotFound(String),
    #[error("papermc.io: version {1} for project {0} not found")]
    PaperMCVersionNotFound(String, String),
    #[error("papermc.io: build {2} for {0} {1} not found")]
    PaperMCBuildNotFound(String, String, String),
}

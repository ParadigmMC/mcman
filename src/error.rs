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
    TomlSerialize(#[from] toml::ser::Error)
}

use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConfigManagerError>;

#[derive(Error, Debug)]
pub enum ConfigManagerError {

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("YAML error: {0}")]
    SerdeYaml(#[from] serde_yaml::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Configuration not found: {0}")]
    ConfigNotFound(String),

    #[error("Invalid configuration format: {0}")]
    InvalidFormat(String),
    
    #[error("Syntax validation error: {0}")]
    SyntaxError(String),

    #[error("File operation error: {0}")]
    FileOperation(String),
    
    #[error("CLI error: {0}")]
    CliError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Directory error: {0}")]
    DirectoryError(String),

    #[error("{0}")]
    Other(String),
}

impl From<String> for ConfigManagerError {
    fn from(s: String) -> Self {
        ConfigManagerError::Other(s)
    }
}

impl From<&str> for ConfigManagerError {
    fn from(s: &str) -> Self {
        ConfigManagerError::Other(s.to_string())
    }
}
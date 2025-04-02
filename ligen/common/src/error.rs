//! Error types.

use thiserror::Error;

/// Library error.
#[derive(Debug, Error)]
pub enum Error {
    /// IO errors.
    #[error("IO error: {0}")]
    IO(std::io::Error),
    /// JSON errors.
    #[error("JSON error: {0}")]
    JSON(serde_json::Error),
    /// Environment errors.
    #[error("Environment error: {0}")]
    Environment(std::env::VarError),
    /// Misc errors.
    #[error("Message: {0}")]
    Message(String),    
    /// Any error.
    #[error("{0}")]
    Anyhow(anyhow::Error),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Message(s.into())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Message(s)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IO(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::JSON(error)
    }
}

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        Self::Environment(error)
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Self::Anyhow(error)
    }
}

/// Library result.
pub type Result<T> = std::result::Result<T, Error>;

//! Error types.

/// Library error.
#[derive(Debug)]
pub enum Error {
    /// IO errors.
    IO(std::io::Error),
    /// JSON errors.
    JSON(serde_json::Error),
    /// Environment errors.
    Environment(std::env::VarError),
    /// Misc errors.
    Message(String),
    /// Generic.
    Generic(Box<dyn std::error::Error>)
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

/// Library result.
pub type Result<T> = std::result::Result<T, Error>;

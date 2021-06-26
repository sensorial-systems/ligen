//! Prelude module with error handling types and others types.

pub(crate) use shrinkwraprs::Shrinkwrap;
pub use proc_macro2::TokenStream;
pub use quote::quote;
pub use quote::TokenStreamExt;

/// Library error.
#[derive(Debug) ]
pub enum Error {
    /// IO errors.
    IO(std::io::Error),
    /// JSON errors.
    JSON(serde_json::Error),
    /// Misc errors.
    Message(String),
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

/// Library result.
pub type Result<T> = std::result::Result<T, Error>;
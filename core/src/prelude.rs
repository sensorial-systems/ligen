//! Prelude module with error handling types and others types.

pub(crate) use shrinkwraprs::Shrinkwrap;

/// Library error.
pub type Error = String;

/// Library result.
pub type Result<T> = std::result::Result<T, Error>;
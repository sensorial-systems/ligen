//! Context about the proc-macro execution.

mod arguments;
mod build_type;
mod source_file;

pub use arguments::*;
pub use build_type::*;
pub use source_file::*;

#[cfg(cargo_ligen)]
use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
/// Context representation.
pub struct Context {
    /// The current SourceFile.
    pub source_file: SourceFile,
    /// Arguments.
    pub arguments: Arguments,
}

impl Context {
    #[cfg(cargo_ligen)]
    /// Get the current generator context by getting the arguments from the environment varilables,
    /// which might fail if they aren't correctly set from `cargo-ligen`.
    pub fn current() -> Result<Self> {
        let source_file = SourceFile::current();
        let arguments = Arguments::from_env()?;
        Ok(Self { source_file, arguments })
    }
}
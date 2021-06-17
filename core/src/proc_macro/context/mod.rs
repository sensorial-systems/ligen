//! Context about the proc-macro execution.

mod arguments;
mod build_type;
mod source_file;

pub use arguments::*;
pub use build_type::*;
pub use source_file::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
/// Context struct.
pub struct Context {
    /// The current SourceFile.
    pub source_file: SourceFile,
    /// Arguments.
    pub arguments: Arguments,
}
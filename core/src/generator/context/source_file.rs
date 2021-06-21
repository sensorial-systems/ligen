//! SourceFile definition module.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
/// SourceFile struct.
pub struct SourceFile {
    /// If it's a real file.
    pub is_real: bool,
    /// The source file path.
    pub path: PathBuf,
}

#[cfg(use_proc_macro)]
impl From<proc_macro::SourceFile> for SourceFile {

    fn from(source_file: proc_macro::SourceFile) -> Self {
        let is_real = source_file.is_real();
        let path = source_file.path();
        Self { is_real, path }
    }
}
//! SourceFile definition module.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[cfg(cargo_ligen)]
use crate::procedural_macro;

#[derive(Debug, Default, Serialize, Deserialize)]
/// SourceFile struct.
pub struct SourceFile {
    is_real: bool,
    path: PathBuf,
}

impl SourceFile {
    #[cfg(cargo_ligen)]
    /// Gets the current source file where the proc-macro is running.
    pub fn current() -> Self {
        procedural_macro::Span::call_site().source_file().into()
    }

    /// If it's a real file.
    pub fn is_real(&self) -> bool { self.is_real }

    /// The source file path.
    pub fn path(&self) -> PathBuf { self.path.clone() }
}

#[cfg(cargo_ligen)]
impl From<procedural_macro::SourceFile> for SourceFile {
    fn from(source_file: procedural_macro::SourceFile) -> Self {
        let path = source_file.path();
        let is_real = source_file.is_real();
        Self { path, is_real }
    }
}
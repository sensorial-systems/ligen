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

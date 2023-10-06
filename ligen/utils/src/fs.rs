//! Helpers for file system operations.

use crate::prelude::*;

use std::path::Path;

/// Creates all the parent directories if they don't exist.
pub fn create_parent_directories(path: &Path) -> Result<()> {
    let file_dir = path
        .parent()
        .ok_or_else(||
            format!("Failed to get the parent of {}.", path.display())
        )?;
    Ok(std::fs::create_dir_all(file_dir)?)
}
/// Writes the file content to the specified path. It creates all the parent directories if they
/// don't exist.
pub fn write_file(path: &Path, content: &String) -> Result<()> {
    create_parent_directories(path)?;
    Ok(std::fs::write(path, content.as_bytes())?)
}

/// Copies the file from the origin path to the destination path. It creates all the parent
/// directories if they don't exist.
pub fn copy(from: &Path, to: &Path) -> Result<()> {
    create_parent_directories(to)?;
    std::fs::copy(from, to)?;
    Ok(())
}
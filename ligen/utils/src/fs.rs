//! Helpers for file system operations.

use crate::prelude::*;

use std::path::Path;

/// Creates all the parent directories if they don't exist.
pub fn create_parent_directories(path: impl AsRef<Path>) -> Result<()> {
    let file_dir = path
        .as_ref()
        .parent()
        .ok_or_else(|| format!("Failed to get the parent of {}.", path.as_ref().display()))?;
    Ok(std::fs::create_dir_all(file_dir)?)
}
/// Writes the file content to the specified path. It creates all the parent directories if they
/// don't exist.
pub fn write_file(path: impl AsRef<Path>, content: impl AsRef<str>) -> Result<()> {
    create_parent_directories(path.as_ref())?;
    Ok(std::fs::write(path.as_ref(), content.as_ref().as_bytes())?)
}

/// Copies the file from the origin path to the destination path. It creates all the parent
/// directories if they don't exist.
pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    create_parent_directories(to.as_ref())?;
    std::fs::copy(from, to)?;
    Ok(())
}
//! File representation.

use crate::prelude::*;
use ligen_utils::fs::write_file;
use std::path::PathBuf;
use std::collections::HashMap;

/// Structure representing a file path and its content.
#[derive(Debug, Clone, PartialEq)]
pub struct File {
    /// File path.
    pub path: PathBuf,
    /// File content.
    pub content: String
}

impl File {
    /// Creates a new file with the specified path and content.
    pub fn new(path: PathBuf, content: String) -> Self {
        Self { path, content }
    }

    /// Writes the content to the file buffer.
    pub fn write<S: AsRef<str>>(&mut self, content: S) {
        self.content.push_str(content.as_ref());
    }

    /// Writes the content to the file buffer and adds a new line.
    pub fn writeln<S: AsRef<str>>(&mut self, content: S) {
        self.content.push_str(content.as_ref());
        self.content.push('\n');
    }

    /// Saves the file.
    pub fn save(&self) -> Result<()> {
        write_file(&self.path, &self.content)
    }
}

/// Structure representing all the file set to be generated.
#[derive(Debug, Default, Clone)]
pub struct FileSet {
    pub(crate) files: HashMap<PathBuf, File>
}

impl FileSet {
    /// Creates a new FileSet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new file.
    pub fn insert(&mut self, file: File) {
        self.files.insert(file.path.clone(), file);
    }

    /// Gets an existing file.
    pub fn get_mut(&mut self, path: &PathBuf) -> Option<&mut File> {
        self.files.get_mut(path)
    }

    /// Returns an existing File assigned to an entry or creates a new one if it isn't present.
    pub fn entry(&mut self, path: &PathBuf) -> &mut File {
        self.files.entry(path.to_path_buf()).or_insert(File::new(path.clone(), Default::default()))
    }
}
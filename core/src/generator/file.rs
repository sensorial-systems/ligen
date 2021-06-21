//! File representation.

use std::path::PathBuf;

/// Structure representing a file path and its content.
#[derive(Debug, Clone)]
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
}

/// Structure representing all the file set to be generated.
#[derive(Debug, Default, Clone)]
pub struct FileSet {
    // FIXME: We need a better API.
    pub(crate) files: Vec<File>
}

impl FileSet {
    /// Creates a new FileSet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new file.
    pub fn add(&mut self, file: File) {
        self.files.push(file)
    }

    /// Gets an existing file.
    pub fn get_mut(&mut self, path: PathBuf) -> Option<&mut File> {
        self.files.iter_mut().find(|file| file.path == path)
    }
}
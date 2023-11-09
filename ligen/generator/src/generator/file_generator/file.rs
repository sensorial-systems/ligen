//! File representation.

use crate::prelude::*;
use ligen_utils::fs::write_file;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, BTreeMap};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct FileSection {
    /// File section content.
    pub content: String
}

impl FileSection {
    /// Creates a new FileSection.
    pub fn new() -> Self {
        Self::default()
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
}

/// Structure representing a file path and its content.
#[derive(Debug, Clone, PartialEq)]
pub struct File {
    /// File path.
    pub path: PathBuf,
    /// File sections.
    pub sections: BTreeMap<String, FileSection>,
    order: Vec<String>
}

impl File {
    /// Creates a new file with the specified path and content.
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        let sections = Default::default();
        let path = path.as_ref().to_path_buf();
        let order = Default::default();
        Self { path, sections, order }
    }

    /// Gets or creates a new section with the specified name.
    pub fn section(&mut self, name: impl AsRef<str>) -> &mut FileSection {
        self
            .sections
            .entry(name.as_ref().to_string())
            .or_insert_with(|| {
                self.order.push(name.as_ref().to_string());
                Default::default()
            })
    }

    /// Gets content.
    pub fn content(&self) -> String {
        let mut content = String::new();
        for section in &self.order {
            if let Some(section) = self.sections.get(section) {
                content.push_str(&section.content);
            }
        }
        content
    }

    /// Saves the file.
    pub fn save(&self) -> Result<()> {
        write_file(&self.path, self.content())
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

    /// Returns an existing File assigned to an entry or creates a new one if it isn't present.
    pub fn entry(&mut self, path: impl AsRef<Path>) -> &mut File {
        self.files.entry(path.as_ref().to_path_buf()).or_insert(File::new(path))
    }
}

#[cfg(test)]
mod tests {
    use super::File;

    #[test]
    fn order() {
        let mut file = File::new("path");
        file.section("b").write("B");
        file.section("a").write("A");
        assert_eq!(file.content(), "BA");
    }
}
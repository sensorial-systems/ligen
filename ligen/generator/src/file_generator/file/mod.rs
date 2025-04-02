use std::{fmt::Debug, path::{PathBuf, Path}};

use crate::prelude::*;

pub mod section;
pub mod set;

pub use section::*;
pub use set::*;

/// Structure representing a file path and its content.
#[derive(Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct File {
    /// File path.
    pub path: PathBuf,
    /// File Section.
    #[shrinkwrap(main_field)]
    pub section: FileSection
}

impl File {
    /// Creates a new file from a template.
    pub fn from_template(path: impl AsRef<Path>, template: &SectionTemplate) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let section = FileSection::from_template(template)?;
        Ok(Self { path, section })
    }

    /// Creates a new file with the specified path and content.
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let section = FileSection::new("root");
        Self { path, section }
    }

    /// Saves the file.
    pub fn save(&self) -> Result<()> {
        ligen_utils::fs::write_file(&self.path, self.to_string())
    }
}

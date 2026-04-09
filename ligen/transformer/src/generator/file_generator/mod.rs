//! File generator module.

mod file;
pub mod template_based;

pub use file::*;
pub use template_based::*;

use crate::prelude::*;

use ligen_utils::fs::write_file;
use std::path::PathBuf;

use crate::generator::Generator;

/// File generator.
pub trait FileGenerator<Input> {
    // TODO: Fetch this from the generator configuration instead and possibly default to something if it doesn't exist.
    /// Generation base path.
    fn base_path(&self) -> PathBuf;

    /// Generate files.
    fn generate_files(&self, input: Input, file_set: &mut FileSet) -> Result<()>;

    /// Saves the file set.
    fn save_file_set(&self, file_set: FileSet, folder: &std::path::Path) -> Result<()> {
        let library_dir = folder.to_path_buf();
        for (_path, file) in file_set.files {
            let file_path = library_dir.join(&file.path);
            write_file(&file_path, file.to_string())?;
        }
        Ok(())
    }
}

impl<I, T: FileGenerator<I>> Generator<I, ()> for T {
    fn generate(&self, input: I, config: &Config) -> Result<()> {
        let mut file_set = FileSet::default();
        self.generate_files(input, &mut file_set)?;
        let output_dir = config
            .get("ligen::output-dir")
            .and_then(|l| l.as_string())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        self.save_file_set(file_set, &output_dir)?;
        Ok(())
    }
}

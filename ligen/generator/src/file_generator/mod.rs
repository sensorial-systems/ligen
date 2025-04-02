//! File generator module.

mod file;
pub mod template_based;

pub use file::*;
pub use template_based::*;

use crate::prelude::*;

use ligen_utils::fs::write_file;
use std::path::PathBuf;

use super::{generator::Generator, generator_config::GeneratorConfig};

/// File generator.
pub trait FileGenerator<Input> {
    // TODO: Fetch this from the generator configuration instead and possibly default to something if it doesn't exist.
    /// Generation base path.
    fn base_path(&self) -> PathBuf;

    /// Generate files.
    fn generate_files(&self, input: &Input, file_set: &mut FileSet) -> Result<()>;

    /// Saves the file set.
    fn save_file_set(&self, file_set: FileSet, folder: &std::path::Path) -> Result<()> {
        let target = folder.to_path_buf();
        let library_dir = target
            .join("ligen")
            .join(self.base_path());
        for (_path, file) in file_set.files {
            let file_path = library_dir.join(&file.path);
            write_file(&file_path, file.to_string())?;
        }
        Ok(())
    }
}

impl <I, T: FileGenerator<I>> Generator<I> for T {
    type Output = ();
    fn generate(&self, _input: &I, _config: &GeneratorConfig) -> Result<Self::Output> {
        todo!("File generator not implemented yet.");
        // let mut file_set = FileSet::default();
        // self.generate_files(input, &mut file_set)?;
        // self.save_file_set(file_set, folder)?;
        // Ok(())
    }
}